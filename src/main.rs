extern crate ruffman;
use ruffman::{compress, decompress};

fn main() {
    let original = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed varius quam ac nisi vestibulum mollis. In posuere purus erat, vulputate vehicula nisl mollis non. Proin finibus, felis faucibus accumsan sagittis, lacus nibh fermentum turpis, at porttitor elit odio in metus. Suspendisse vel ipsum nisl. Duis convallis porta accumsan. Nulla faucibus congue arcu, a efficitur eros imperdiet sed. Mauris vel rhoncus velit, eu facilisis ex. Nam faucibus leo vitae erat facilisis cursus. Maecenas pulvinar arcu ut nibh volutpat, sit amet dictum felis efficitur. Curabitur ut odio quis ante efficitur accumsan sit amet eget ipsum. ");

    let compressed = compress(&original);
    print!("{}", decompress(compressed));
}
