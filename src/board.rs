use dominator::{
    Dom, DomBuilder,
    __internal::{HtmlElement, SvgElement},
    class,
    events::MouseDown,
    html, svg,
};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use once_cell::sync::Lazy;

use crate::{
    card::{get_offset, in_card},
    render::{Render, RenderOpt},
    App,
};

#[derive(Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece(pub Player, pub PieceKind);

#[derive(Clone, Copy, PartialEq)]
pub enum Overlay {
    Highlight,
    Dot,
}

static OVERLAY_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
    }
});

impl App {
    pub fn render_square(&self, pos: usize) -> Dom {
        static SPAN_DARK: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-block")
                .style("vertical-align", "bottom")
                .style("background", "#b58863")
                .style("width", "80px")
                .style("height", "80px")
            }
        });

        static SPAN_LIGHT: Lazy<String> = Lazy::new(|| {
            class! {
                .style("display", "inline-block")
                .style("vertical-align", "bottom")
                .style("background", "#f0d9b5")
                .style("width", "80px")
                .style("height", "80px")
            }
        });

        let selected = self.selected.clone();
        let square = self.board[pos].clone();

        html!("span", {
            .class(if pos % 2 == 1 {
                &*SPAN_DARK
            } else {
                &*SPAN_LIGHT
            })
            .event(move |_: MouseDown|{
                let s = selected.get();
                let square = square.get();
                if s != Some(pos) && square.is_some() && square.unwrap().0 == Player::White {
                    selected.set(Some(pos));
                } else {
                    selected.set(None)
                }
            })
            .apply(|mut dom| {
                for player in [Player::Black, Player::White] {
                    for kind in [PieceKind::Pawn, PieceKind::King] {
                        let piece = Piece(player, kind);
                        dom = dom.child(html!("img", {
                            .class(&*OVERLAY_CLASS)
                            .visible_signal(
                                self.board[pos].signal().map(move |p|p == Some(piece))
                            )
                            .apply(|dom|piece.render(dom))
                        }))
                    }
                };
                for overlay in [Overlay::Highlight, Overlay::Dot] {
                    dom = dom.child(svg!("svg", {
                        .class(&*OVERLAY_CLASS)
                        .visible_signal(
                            self.get_overlay(pos).map(move |o|o==Some(overlay))
                        )
                        .apply(|dom|overlay.render(dom))
                    }))
                }
                dom
            })
        })
    }

    fn get_overlay(&self, pos: usize) -> impl Signal<Item = Option<Overlay>> {
        let (card1, card2) = (self.cards[0].clone(), self.cards[1].clone());
        let square = self.board[pos].clone();
        self.selected.signal_ref(move |&from| {
            let offset = get_offset(pos, from?)?;
            let possible = in_card(offset, card1.get()) || in_card(offset, card2.get());
            let square = square.get();
            if offset == 12 {
                Some(Overlay::Highlight)
            } else if possible && (square.is_none() || square.unwrap().0 != Player::White) {
                Some(Overlay::Dot)
            } else {
                None
            }
        })
    }
}

impl Piece {
    fn render(&self, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        let file = match self {
            Piece(Player::White, PieceKind::King) => "wB.svg",
            Piece(Player::White, PieceKind::Pawn) => "wP.svg",
            Piece(Player::Black, PieceKind::King) => "bB.svg",
            Piece(Player::Black, PieceKind::Pawn) => "bP.svg",
        };

        dom.attr("src", file)
            .attr("width", "80")
            .attr("height", "80")
    }
}

impl Overlay {
    fn render(&self, dom: DomBuilder<SvgElement>) -> DomBuilder<SvgElement> {
        match self {
            Overlay::Highlight => dom
                .attr("width", "80")
                .attr("height", "80")
                .attr("viewBox", "0 0 1 1")
                .child(svg!("rect", {
                    .attr("fill", "none")
                    .attr("stroke", "green")
                    .attr("stroke-width", "0.1")
                    .attr("width", "1")
                    .attr("height", "1")
                    .attr("stroke-dasharray", "0.5")
                    .attr("stroke-dashoffset", "0.25")
                    .attr("stroke-opacity", "0.5")
                })),
            Overlay::Dot => dom
                .attr("width", "80")
                .attr("height", "80")
                .attr("viewBox", "-1 -1 2 2")
                .child(svg!("circle", {
                    .attr("r", "0.25")
                    .attr("fill", "green")
                    .attr("fill-opacity", "0.5")
                })),
        }
    }
}
