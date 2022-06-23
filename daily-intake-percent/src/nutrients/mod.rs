use crate::pretty::pretty;
use std::collections::HashSet;
use std::fmt;

mod amino_acid;
mod carbs_and_sugars;
mod fatty_acid;
mod macronutrient;
mod mineral;
mod vitamin;

/// A trait implemented on each nutrient category. Used by the Nutrients struct
/// for dynamic dispatch.
pub trait NutrientType {
    /// Returns the name of the nutrient category.
    fn get_name(&self) -> &str;

    /// Returns the nutrients in the category.
    fn get_nutrients(&self) -> Vec<&Nutrient>;

    /// Returns the mutable nutrients in the category.
    fn get_nutrients_mut(&mut self) -> Vec<&mut Nutrient>;

    /// Displays a table of the nutrients in the category.
    /// Overriding this function isn't recommended as its default is sensible
    /// and gives consistency, but can be done so if necessary.
    ///
    /// Only prints out the nutrients with a non-zero value.
    ///
    /// `best_percent` is the daily intake percentage of the best nutrient
    /// entered out of anything in the Nutrients struct. It is used for
    /// colouring the output table.
    ///
    /// `f` is the [`Formatter`] for the output table.
    ///
    /// [`Formatter`]: https://doc.rust-lang.org/std/fmt/struct.Formatter.html
    fn display(&self, best_percent: f64, f: &mut fmt::Formatter) -> fmt::Result {
        // Center align with bold underline the name of the nutrient category
        writeln!(f, "{:<60}", pretty!(self.get_name(), b | _))?;

        // If all nutrients in this table are empty
        let percentages = self
            .get_nutrients()
            .iter()
            .map(|nutrient| nutrient.percent())
            .collect::<Vec<f64>>();
        if percentages.iter().all(|x| *x == 0f64) {
            writeln!(f, "{:^60}", pretty!("Nothing", b))?;
            return Ok(());
        }

        writeln!(
            f,
            "{}",
            pretty!(
                format!(
                    "{:^20} │ {:^10} │ {:^10} │ {:^8}",
                    "Nutrient", "DV", "Each Serve", "DV%"
                ),
                b | _
            )
        )?;

        for nutrient in self.get_nutrients() {
            let percent = nutrient.percent();

            if percent != 0. {
                let text = format!("{}", nutrient);
                writeln!(
                    f,
                    "{:>20}",
                    if percent > best_percent * 0.7 {
                        pretty!(text, b + fg@green)
                    } else if percent > best_percent * 0.3 {
                        pretty!(text, b + fg@yellow)
                    } else {
                        pretty!(text, b + fg@red)
                    }
                )?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for dyn NutrientType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.display(0., f)
    }
}

#[derive(Default)]
pub struct Nutrients<'a> {
    macros: macronutrient::Macronutrient<'a>,
    vitamins: vitamin::Vitamin<'a>,
    minerals: mineral::Mineral<'a>,
    carbs_and_sugars: carbs_and_sugars::CarbsAndSugar<'a>,
    fatty_acids: fatty_acid::FattyAcid<'a>,
    amino_acids: amino_acid::AminoAcid<'a>,
}

impl<'a> Nutrients<'a> {
    pub fn get_nutrient_types(&self) -> Vec<&dyn NutrientType> {
        vec![
            &self.macros,
            &self.vitamins,
            &self.minerals,
            &self.carbs_and_sugars,
            &self.fatty_acids,
            &self.amino_acids,
        ]
    }

    pub fn get_nutrient_types_mut(&mut self) -> Vec<&mut dyn NutrientType> {
        vec![
            &mut self.macros,
            &mut self.vitamins,
            &mut self.minerals,
            &mut self.carbs_and_sugars,
            &mut self.fatty_acids,
            &mut self.amino_acids,
        ]
    }

    pub fn get_nutrient(&self, alias: &str) -> Option<&Nutrient> {
        let alias = alias.to_lowercase();
        for nutrient_type in self.get_nutrient_types() {
            for nutrient in nutrient_type.get_nutrients() {
                if nutrient.has_alias(&alias) {
                    return Some(nutrient);
                }
            }
        }
        None
    }

    pub fn get_nutrient_mut(&mut self, alias: &str) -> Option<&mut Nutrient> {
        let alias = alias.to_lowercase();
        for nutrient_type in self.get_nutrient_types_mut() {
            for nutrient in nutrient_type.get_nutrients_mut() {
                if nutrient.has_alias(&alias) {
                    return Some(nutrient);
                }
            }
        }
        None
    }

    pub fn table_with_shortest_alias(&self) -> String {
        let mut s = format!(
            "{}\n",
            pretty!(format!("{:^78}", "Nutrient Table"), b | i | _)
        );

        let table_header = format!(
            "\n{}",
            pretty!(
                format!(
                    "{:^20} {:^20} │ {:^10} │ {:^10} │ {:^8}",
                    "Nutrient", "Abbr", "DV", "Each Serve", "DV%"
                ),
                b | _
            )
        );

        for nutrient_type in self.get_nutrient_types() {
            s += &format!(
                "\n{}",
                pretty!(format!("{:^78}", nutrient_type.get_name()), b)
            );
            s += &table_header.clone();
            for nutrient in nutrient_type.get_nutrients() {
                s += &format!("\n{}", nutrient.display_with_shortest_alias());
            }
            s += "\n";
        }

        s
    }

    pub fn table_with_shortest_alias_highlight(&self, alias: &str) -> String {
        let mut s = format!(
            "{}\n",
            pretty!(format!("{:^78}", "Nutrient Table"), b | i | _)
        );

        let table_header = format!(
            "\n{}",
            pretty!(
                format!(
                    "{:^20} {:^20} │ {:^10} │ {:^10} │ {:^8}",
                    "Nutrient", "Abbr", "DV", "Each Serve", "DV%"
                ),
                b | _
            )
        );

        for nutrient_type in self.get_nutrient_types() {
            s += &format!(
                "\n{}",
                pretty!(format!("{:^78}", nutrient_type.get_name()), b)
            );
            s += &table_header.clone();
            for nutrient in nutrient_type.get_nutrients() {
                let display = if nutrient.has_alias(alias) {
                    format!(
                        "{}",
                        pretty!(nutrient.display_with_shortest_alias(), b + fg@green)
                    )
                } else {
                    nutrient.display_with_shortest_alias().to_string()
                };

                s += &format!("\n{}", display);
            }
            s += "\n";
        }

        s
    }
}

impl<'a> fmt::Display for Nutrients<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut best_percent: f64 = 0.;
        for nutrient_type in self.get_nutrient_types() {
            for nutrient in nutrient_type.get_nutrients() {
                best_percent = best_percent.max(nutrient.percent());
            }
        }

        writeln!(
            f,
            "{}",
            pretty!(format!("{:^78}", "Nutrient Table"), b | i | _)
        )?;

        for nutrient_type in self.get_nutrient_types() {
            writeln!(f)?;
            nutrient_type.display(best_percent, f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
#[readonly::make]
pub struct Nutrient<'a> {
    pub name: &'a str,
    pub short: Option<String>,
    pub aliases: HashSet<String>,
    pub unit: &'a str,
    pub value: Option<f64>,
    pub daily_value: f64,
}

impl<'a> Nutrient<'a> {
    pub fn new(name: &'a str, aliases: &'a [&'a str], unit: &'a str, daily_value: f64) -> Self {
        let mut aliases = aliases.to_vec();
        aliases.push(name);
        let aliases: HashSet<String> = aliases.iter().map(|s| s.to_lowercase()).collect();
        Self {
            name,
            unit,
            // Shortest alias
            short: aliases
                .iter()
                .min_by_key(|s| s.len())
                .map(|s| s.to_string()),
            aliases,
            value: None,
            daily_value,
        }
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = Some(value);
    }

    pub fn has_alias(&self, alias: &str) -> bool {
        self.aliases.contains(&alias.to_string())
    }

    pub fn percent(&self) -> f64 {
        if self.daily_value != 0. {
            self.value.unwrap_or(0.) / self.daily_value
        } else {
            0.
        }
    }

    pub fn display_with_shortest_alias(&self) -> String {
        let abbr = format!("({})", self.short.as_ref().unwrap());

        let dv = format!("{}{}", self.daily_value, self.unit);
        let value = if let Some(value) = self.value {
            format!("{}{}", value, self.unit)
        } else {
            "".to_string()
        };

        format!(
            "{:>20}  {:<19} │  {:<8}  │  {:<8}  │ {:7.2}%",
            self.name,
            abbr,
            dv,
            value,
            self.percent() * 100.
        )
    }
}

impl<'a> fmt::Display for Nutrient<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dv = format!("{}{}", self.daily_value, self.unit);
        let value = if let Some(value) = self.value {
            format!("{}{}", value, self.unit)
        } else {
            "".to_string()
        };

        write!(
            f,
            "{:>20} │  {:<8}  │  {:<8}  │ {:7.2}%",
            self.name,
            dv,
            value,
            self.percent() * 100.
        )
    }
}
