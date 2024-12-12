use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "iphone 14Pro Max".to_string(),
            price: 9.99,
            description: "Apple's iPhone is renowned for its sleek design, powerful performance, and robust ecosystem. It features advanced camera systems, smooth iOS integration, and a wide range of applications, making it a top choice for both personal and professional use.".to_string(),
            image: "/bestbuymobile-phone.png".to_string()
        },
        Product {
            id: 2,
            name: "Airpods".to_string(),
            price: 6.99,
            description: "Apple AirPods are wireless Bluetooth earbuds designed for seamless integration with Apple devices, featuring high-quality audio, instant connectivity, and voice-activated Siri access. They are known for their comfort, battery life, and the convenience of the charging case that comes with them.".to_string(),
            image: "/bestbuymobile-earphones.png".to_string()
        },
        Product {
            id: 3,
            name: "Laptop".to_string(),
            price: 12.99,
            description: "The HP Spectre series features sleek, high-performance laptops with stunning displays and powerful processors, ideal for professionals and creatives who demand efficiency and style. They typically offer long battery life, robust security features, and a lightweight design for superb portability.".to_string(),
            image: "/bestbuy-laptop.png".to_string()
        },
       
    ]
}