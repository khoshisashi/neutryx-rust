//! Screen rendering functions for the TUI.

use crate::app::{RiskMetrics, TradeRow};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

/// Format a number with thousands separators
fn format_number(n: f64, decimals: usize) -> String {
    if decimals == 0 {
        format!("{:.0}", n)
    } else {
        format!("{:.1$}", n, decimals)
    }
}

/// Draw dashboard screen
pub fn draw_dashboard(frame: &mut Frame, area: Rect, metrics: &RiskMetrics) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Portfolio summary
    let portfolio_text = vec![
        Line::from(vec![
            Span::raw("Total PV: "),
            Span::styled(
                format_number(metrics.total_pv, 2),
                Style::default().fg(if metrics.total_pv >= 0.0 {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::raw("XVA Adjustments:")]),
        Line::from(vec![
            Span::raw("  CVA: "),
            Span::styled(format_number(metrics.cva, 2), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("  DVA: "),
            Span::styled(
                format_number(metrics.dva, 2),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("  FVA: "),
            Span::styled(
                format_number(metrics.fva, 2),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let portfolio = Paragraph::new(portfolio_text)
        .block(Block::default().title(" Portfolio Summary ").borders(Borders::ALL));
    frame.render_widget(portfolio, chunks[0]);

    // Risk metrics
    let risk_text = vec![
        Line::from(vec![
            Span::raw("Expected Exposure (EE): "),
            Span::styled(format_number(metrics.ee, 2), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Expected Positive Exp: "),
            Span::styled(
                format_number(metrics.epe, 2),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Potential Future Exp:  "),
            Span::styled(
                format_number(metrics.pfe, 2),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let risk = Paragraph::new(risk_text)
        .block(Block::default().title(" Exposure Metrics ").borders(Borders::ALL));
    frame.render_widget(risk, chunks[1]);
}

/// Draw portfolio screen
pub fn draw_portfolio(frame: &mut Frame, area: Rect, trades: &[TradeRow], selected: usize) {
    let header_cells = ["ID", "Instrument", "Notional", "PV", "Delta", "Gamma", "Vega"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells).height(1);

    let rows = trades.iter().enumerate().map(|(idx, trade)| {
        let style = if idx == selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(trade.id.clone()),
            Cell::from(trade.instrument.clone()),
            Cell::from(format_number(trade.notional, 0)),
            Cell::from(format_number(trade.pv, 2)).style(Style::default().fg(
                if trade.pv >= 0.0 {
                    Color::Green
                } else {
                    Color::Red
                },
            )),
            Cell::from(format!("{:.4}", trade.delta)),
            Cell::from(format!("{:.4}", trade.gamma)),
            Cell::from(format!("{:.4}", trade.vega)),
        ])
        .style(style)
    });

    let widths = [
        Constraint::Length(8),
        Constraint::Min(20),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title(" Portfolio ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(table, area);
}

/// Draw risk screen
pub fn draw_risk(frame: &mut Frame, area: Rect, metrics: &RiskMetrics) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // XVA metrics
    let xva_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  CVA (Credit Value Adjustment):    ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.cva),
                Style::default().fg(Color::Red),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  DVA (Debit Value Adjustment):     ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.dva),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  FVA (Funding Value Adjustment):   ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.fva),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Total XVA:                        ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.cva + metrics.dva + metrics.fva),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    let xva = Paragraph::new(xva_text)
        .block(Block::default().title(" XVA Metrics ").borders(Borders::ALL));
    frame.render_widget(xva, chunks[0]);

    // Exposure metrics
    let exposure_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  EE (Expected Exposure):           ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.ee),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  EPE (Expected Positive Exposure): ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.epe),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  PFE (Potential Future Exposure):  ",
                Style::default().fg(Color::White),
            ),
            Span::styled(
                format!("{:>12.2}", metrics.pfe),
                Style::default().fg(Color::Yellow),
            ),
        ]),
    ];

    let exposure = Paragraph::new(exposure_text)
        .block(Block::default().title(" Exposure Metrics ").borders(Borders::ALL));
    frame.render_widget(exposure, chunks[1]);
}

/// Draw trade blotter screen
pub fn draw_trade_blotter(frame: &mut Frame, area: Rect, trade: Option<&TradeRow>) {
    let content = if let Some(t) = trade {
        vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Trade ID:    ", Style::default().fg(Color::Yellow)),
                Span::raw(&t.id),
            ]),
            Line::from(vec![
                Span::styled("Instrument:  ", Style::default().fg(Color::Yellow)),
                Span::raw(&t.instrument),
            ]),
            Line::from(vec![
                Span::styled("Notional:    ", Style::default().fg(Color::Yellow)),
                Span::raw(format_number(t.notional, 2)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "--- Valuation ---",
                Style::default().fg(Color::Cyan),
            )]),
            Line::from(vec![
                Span::styled("PV:          ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    format_number(t.pv, 2),
                    Style::default().fg(if t.pv >= 0.0 {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "--- Greeks ---",
                Style::default().fg(Color::Cyan),
            )]),
            Line::from(vec![
                Span::styled("Delta:       ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.6}", t.delta)),
            ]),
            Line::from(vec![
                Span::styled("Gamma:       ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.6}", t.gamma)),
            ]),
            Line::from(vec![
                Span::styled("Vega:        ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.6}", t.vega)),
            ]),
        ]
    } else {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "No trade selected",
                Style::default().fg(Color::DarkGray),
            )),
        ]
    };

    let blotter = Paragraph::new(content)
        .block(Block::default().title(" Trade Details ").borders(Borders::ALL));
    frame.render_widget(blotter, area);
}
