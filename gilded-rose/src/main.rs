trait InnItem: std::fmt::Debug {
    fn sell_in(&self) -> u32;
    fn age(&self) -> u32;
    fn quality(&self) -> u32;
    fn age_mut(&mut self) -> &mut u32;
    fn quality_mut(&mut self) -> &mut u32;
    fn degrade(&mut self) {
        *self.age_mut() = self.age() + 1;

        let degrade = if self.age() > self.sell_in() { 2 } else { 1 };
        if self.quality() > 0 {
            *self.quality_mut() = self.quality().saturating_sub(degrade);
        };
    }
}

#[derive(Debug)]
struct InnItemBase {
    age: u32,
    quality: u32,
}

#[derive(Debug)]
struct DexterityVest {
    inner: InnItemBase,
}

impl DexterityVest {
    fn new() -> Self {
        Self {
            inner: InnItemBase {
                age: 0,
                quality: 30,
            },
        }
    }
}

impl InnItem for DexterityVest {
    fn sell_in(&self) -> u32 {
        30
    }
    fn age(&self) -> u32 {
        self.inner.age
    }
    fn quality(&self) -> u32 {
        self.inner.quality
    }
    fn age_mut(&mut self) -> &mut u32 {
        &mut self.inner.age
    }
    fn quality_mut(&mut self) -> &mut u32 {
        &mut self.inner.quality
    }
}

#[derive(Debug)]
struct AgedBrie {
    inner: InnItemBase,
}

impl AgedBrie {
    fn new() -> Self {
        Self {
            inner: InnItemBase {
                age: 0,
                quality: 10,
            },
        }
    }
}

impl InnItem for AgedBrie {
    fn sell_in(&self) -> u32 {
        0
    }
    fn age(&self) -> u32 {
        self.inner.age
    }
    fn quality(&self) -> u32 {
        self.inner.quality
    }
    fn age_mut(&mut self) -> &mut u32 {
        &mut self.inner.age
    }
    fn quality_mut(&mut self) -> &mut u32 {
        &mut self.inner.quality
    }
    fn degrade(&mut self) {
        self.inner.age += 1;
        if self.inner.quality < 50 {
            self.inner.quality += 1;
        }
    }
}

#[derive(Debug)]
struct Sulfuras {
    inner: InnItemBase,
}

impl Sulfuras {
    fn new() -> Self {
        Self {
            inner: InnItemBase {
                age: 0,
                quality: 45,
            },
        }
    }
}

impl InnItem for Sulfuras {
    fn sell_in(&self) -> u32 {
        0
    }
    fn age(&self) -> u32 {
        self.inner.age
    }
    fn quality(&self) -> u32 {
        self.inner.quality
    }
    fn age_mut(&mut self) -> &mut u32 {
        &mut self.inner.age
    }
    fn quality_mut(&mut self) -> &mut u32 {
        &mut self.inner.quality
    }
    fn degrade(&mut self) {
        self.inner.age += 1;
    }
}

#[derive(Debug)]
struct Bread {
    inner: InnItemBase,
}

impl Bread {
    fn new() -> Self {
        Self {
            inner: InnItemBase {
                age: 0,
                quality: 20,
            },
        }
    }
}

impl InnItem for Bread {
    fn sell_in(&self) -> u32 {
        5
    }
    fn age(&self) -> u32 {
        self.inner.age
    }
    fn quality(&self) -> u32 {
        self.inner.quality
    }
    fn age_mut(&mut self) -> &mut u32 {
        &mut self.inner.age
    }
    fn quality_mut(&mut self) -> &mut u32 {
        &mut self.inner.quality
    }
}

fn main() {
    let mut inventory: Vec<Box<dyn InnItem>> = vec![
        Box::new(DexterityVest::new()),
        Box::new(AgedBrie::new()),
        Box::new(Sulfuras::new()),
        Box::new(Bread::new()),
    ];

    for i in 0..50 {
        inventory.iter_mut().for_each(|elem| {
            elem.degrade();
            println!("{}. day have {:?}", i, elem);
        });
    }
}
