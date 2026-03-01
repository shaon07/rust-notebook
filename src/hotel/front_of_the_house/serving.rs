use crate::hotel::back_of_the_house::Breakfast;
fn take_order() {
    println!("Order Placed");
}

pub fn serve_order() {
    take_order();
    take_payment();
    println!("Start Serving...");
    let order: Breakfast = Breakfast::summer("Wheat");
    println!("Order item is {} and free", order.toast);
}

fn take_payment() {
    println!("Taking payment onging...");
}
