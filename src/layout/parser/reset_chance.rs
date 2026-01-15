use super::{GradientBuilder, Result, color, end_tag, parse_bool, parse_children};

pub use crate::component::reset_chance::Component;
use crate::util::xml::Reader;

pub fn settings(reader: &mut Reader, component: &mut Component) -> Result<()> {
    let settings = component.settings_mut();
    let mut background_builder = GradientBuilder::new();
    let (mut override_label, mut override_value) = (false, false);

    parse_children(reader, |reader, tag, _| {
        if !background_builder.parse_background(reader, tag.name())? {
            match tag.name() {
                "TextColor" => color(reader, |c| settings.label_color = Some(c)),
                "OverrideTextColor" => parse_bool(reader, |b| override_label = b),
                "ChanceColor" => color(reader, |c| settings.value_color = Some(c)),
                "OverrideChanceColor" => parse_bool(reader, |b| override_value = b),
                "Display2Rows" => parse_bool(reader, |b| settings.display_two_rows = b),
                _ => {
                    // Unsupported:
                    // ChanceMode
                    // Accuracy
                    // Basis, BasisSubset, BasisSubsetSplits
                    end_tag(reader)
                }
            }
        } else {
            Ok(())
        }
    })?;

    if !override_label {
        settings.label_color = None;
    }
    if !override_value {
        settings.value_color = None;
    }
    settings.background = background_builder.build();

    Ok(())
}
