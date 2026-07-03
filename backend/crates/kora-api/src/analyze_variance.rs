use kora_domain::agriculture::drift::{
    VarianceReport as TimingReport, EconomicVarianceReport, VarianceConfig,
    VarianceService, EconomicVarianceService,
};
use kora_domain::agriculture::planning::Schedule;
use kora_domain::agriculture::cycle::CropCycle;
use kora_domain::ports::economic_data_provider::EconomicDataProvider;

pub struct AnalyzeVarianceInput {
    pub schedule: Schedule,
    pub cycle: CropCycle,
    pub config: VarianceConfig,
    pub economic_provider: Option<Box<dyn EconomicDataProvider>>,
}

pub struct AnalyzeVarianceOutput {
    pub timing_report: TimingReport,
    pub economic_report: Option<EconomicVarianceReport>,
}

pub fn execute(input: AnalyzeVarianceInput) -> AnalyzeVarianceOutput {
    let timing_report = VarianceService::analyze_with_config(
        &input.schedule,
        &input.cycle,
        &input.config,
    );

    let economic_report = input.economic_provider.as_ref().map(|provider| {
        EconomicVarianceService::analyze_costs(&timing_report.matched, provider.as_ref())
    });

    AnalyzeVarianceOutput { timing_report, economic_report }
}
