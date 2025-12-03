use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    ParseInt(char),
}

impl From<char> for ParseError {
    fn from(value: char) -> Self {
        Self::ParseInt(value)
    }
}

type Battery = u64;
type Joltage = u64;

const BATTERY_RADIX: u32 = 10;
const MIN_JOLTAGE: Joltage = 1;
const JOLT_MULTIPLIER_BASE: Joltage = 10;

pub struct EmergencyPower {
    banks: Vec<BatteryBank>,
}

impl FromStr for EmergencyPower {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let banks = s
            .lines()
            .map(BatteryBank::from_str)
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { banks })
    }
}

impl EmergencyPower {
    #[inline(always)]
    pub fn max_joltage_unsafe<const AMOUNT: usize>(&self) -> Joltage {
        self.banks
            .iter()
            .map(BatteryBank::max_joltage::<AMOUNT>)
            .sum()
    }

    #[inline(always)]
    pub fn max_joltage(&self) -> Joltage {
        self.banks.iter().map(BatteryBank::max_joltage::<2>).sum()
    }
}

pub struct BatteryBank {
    batteries: Vec<Battery>,
}

impl FromStr for BatteryBank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .trim()
            .chars()
            .map(|c| c.to_digit(BATTERY_RADIX).map(|n| n as Battery).ok_or(c))
            .collect::<Result<Vec<_>, char>>()?;

        Ok(Self { batteries })
    }
}

impl BatteryBank {
    pub fn max_joltage<const AMOUNT: usize>(&self) -> Joltage {
        let joltages = self.batteries.windows(AMOUNT).fold(
            [MIN_JOLTAGE; AMOUNT],
            |mut joltages, battery_joltages| {
                (0..(AMOUNT - 1)).for_each(|pos| {
                    if joltages[pos] < battery_joltages[pos] {
                        joltages[pos] = battery_joltages[pos];
                        joltages[pos + 1] = MIN_JOLTAGE;
                    }
                });

                if joltages[AMOUNT - 1] < battery_joltages[AMOUNT - 1] {
                    joltages[AMOUNT - 1] = battery_joltages[AMOUNT - 1];
                }

                joltages
            },
        );

        joltages
            .into_iter()
            .rev()
            .enumerate()
            .map(|(exp, jolt)| {
                let multiplier = JOLT_MULTIPLIER_BASE.pow(exp as u32);
                jolt * multiplier
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn solution_1() {
        let emergency_power = EmergencyPower::from_str(EXAMPLE).unwrap();
        assert_eq!(emergency_power.max_joltage(), 357)
    }

    #[test]
    fn solution_2() {
        let emergency_power = EmergencyPower::from_str(EXAMPLE).unwrap();
        assert_eq!(emergency_power.max_joltage_unsafe::<12>(), 3121910778619);
    }
}
