use ordered_float::NotNan;

#[derive(Debug, PartialEq, Eq)]
pub struct WordMeaning {
    class: WordMeaningClass,
    likelihoods: Likelihoods,
}

impl WordMeaning {
    pub fn class(&self) -> WordMeaningClass {
        self.class
    }

    pub fn likelihood(&self, channel: LikelihoodChannel) -> Likelihood {
        self.likelihoods.likelihood(channel)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WordMeaningClass {
    Adjective,
    Noun,
    Determiner,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Likelihoods {
    in_math: Likelihood,
}

impl Likelihoods {
    fn likelihood(&self, channel: LikelihoodChannel) -> Likelihood {
        match channel {
            LikelihoodChannel::Math => self.in_math,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Likelihood {
    log_value: NotNan<f32>,
}

pub enum LikelihoodChannel {
    Math,
}
