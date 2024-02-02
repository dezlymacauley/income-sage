use core::num;

use slint::SharedString;

slint::include_modules!();

const TAXPERCENTAGE: f64 = 0.30; // 30%
const OWNERPERCENTAGE: f64 = 0.55; // 55%
const PROFITPERCENTAGE: f64 = 0.05; // 5%
const OPERATINGEXPENSES: f64 = 0.10; // 10%

// The main function returns a result,
// and declares the App Window component
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    
    // Next a "Weak Pointer is created to it"
    let ui_handle: Weak<AppWindow> =  ui.as_weak();
    ui.on_divide_income(move | string: SharedString | {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
       
        let tax: f64 = num * TAXPERCENTAGE;
        let owner: f64 = num * OWNERPERCENTAGE;
        let profit: f64 = num * PROFITPERCENTAGE;
        let opex: f64 = num * OPERATINGEXPENSES;
        
        // {:.2} means format to 2 decimal places
        let result: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperatingExpenses: {:.2}", tax, owner, profit, opex);
        ui.set_results(result.into());
    });

    ui.run()

}
