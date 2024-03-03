use slint::SharedString;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax = num * TAXPER;
        let owner = num * OWNERPER;
        let profit = num * PROFITPER;
        let opex = num * OPEXPER;
        
        let result: String = format!("Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", tax, owner, profit, opex);
        ui.set_results(result.into());
    });

    ui.run()
}

