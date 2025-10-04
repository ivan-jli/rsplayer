pub mod ssd1306;
pub mod st7920;

use api_models::settings::OLEDModel::{SSD1306_128x64, ST7920_128x64};
use api_models::state::StateChangeEvent;
use rsplayer_config::ArcConfiguration;
use tokio::sync::broadcast::Receiver;

pub async fn write(state_changes_rx: Receiver<StateChangeEvent>, config: ArcConfiguration) {
    let settings = config.get_settings();
    if !settings.oled_settings.enabled {
        crate::common::logging_receiver_future(state_changes_rx).await;
    } else {
        match settings.oled_settings.display_model {
            ST7920_128x64 => st7920::write(state_changes_rx, config).await,
            SSD1306_128x64 => ssd1306::write(state_changes_rx, config).await,
        }
    }
}
