use common::{Action, SwapEvent, events::BotEvent};
use tracing::info;

pub trait Strategy: Send {
    fn on_event(&mut self, event: &SwapEvent) -> Action;
    fn on_ai_signal(&mut self, signal: &BotEvent);
}

pub struct SimpleStrategy {
    threshold: u128,
    last_ai_confidence: f64,
}

impl SimpleStrategy {
    pub fn new(threshold: u128) -> Self {
        Self { 
            threshold,
            last_ai_confidence: 1.0, // Default to trusting 
        }
    }
}

impl Strategy for SimpleStrategy {
    fn on_event(&mut self, event: &SwapEvent) -> Action {
        // [VETO LOGIC] - Institutional AI Confidence Gate
        if self.last_ai_confidence < 0.65 {
            info!("Strategy Veto: AI Confidence {} is below 0.65 threshold.", self.last_ai_confidence);
            return Action::Hold;
        }

        if event.amount_in > self.threshold {
            Action::Buy {
                token: event.token_out.clone(),
                size: 0.1,
                confidence: 0.9,
            }
        } else {
            Action::Hold
        }
    }

    fn on_ai_signal(&mut self, event: &BotEvent) {
        if let BotEvent::AISignal { confidence, symbol, .. } = event {
            info!("Strategy engine ingesting AI Signal for {}. Confidence: {}", symbol, confidence);
            self.last_ai_confidence = *confidence;
        }
    }
}
