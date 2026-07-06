use serde::Serialize;

#[derive(Serialize)]
pub struct HomeResponse {
    pub greeting: String,
    pub date: String,
    pub today: TodaySection,
    pub fields: Vec<HomeFieldPreview>,
    pub team: TeamPreview,
    pub finances: FinancePreview,
    pub alerts: Vec<AlertItem>,
    pub weather: Option<WeatherInfo>,
}

#[derive(Serialize)]
pub struct TodaySection {
    pub pending: u32,
    pub critical: u32,
    pub completed: u32,
    pub next_action: Option<NextActionData>,
}

#[derive(Serialize)]
pub struct NextActionData {
    pub title: String,
    pub field: String,
    pub crop: String,
    pub when: String,
    pub priority: String,
}

#[derive(Serialize)]
pub struct HomeFieldPreview {
    pub id: String,
    pub name: String,
    pub crop: String,
    pub hectares: f64,
    pub progress_percent: u32,
    pub health: String,
    pub days_to_harvest: i64,
    pub last_activity: String,
    pub next_activity: String,
}

#[derive(Serialize)]
pub struct TeamPreview {
    pub total: usize,
    pub working_today: usize,
    pub missing_yesterday: usize,
    pub entries: Vec<TeamEntry>,
}

#[derive(Serialize)]
pub struct TeamEntry {
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct FinancePreview {
    pub total_budget: String,
    pub budget_used_percent: u32,
    pub total_spent: String,
    pub alerts: Vec<FinanceAlert>,
}

#[derive(Serialize)]
pub struct FinanceAlert {
    pub cycle: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct AlertItem {
    pub kind: String,
    pub text: String,
    pub field: Option<String>,
    pub severity: String,
}

#[derive(Serialize)]
pub struct WeatherInfo {
    pub forecast: String,
    pub rain_expected: bool,
    pub temp: String,
    pub humidity: String,
}
