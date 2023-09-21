use std::ops;
use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::ptr;

#[derive(Copy, Clone)]
enum WireValue {
    ZERO,
    ONE,
    Z,
    ERR,
}

impl ops::BitAnd for WireValue {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (WireValue::ZERO, WireValue::ZERO) => WireValue::ZERO,
            (WireValue::ZERO, WireValue::ONE) => WireValue::ZERO,
            (WireValue::ONE, WireValue::ZERO) => WireValue::ZERO,
            (WireValue::ONE, WireValue::ONE) => WireValue::ONE,
            _ => WireValue::Z
        }
    }
}

impl ops::Not for WireValue {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            WireValue::ZERO => WireValue::ONE,
            WireValue::ONE => WireValue::ZERO,
            _ => self
        }
    }
}

struct ComponentRef(Rc<dyn Component>);

impl Hash for ComponentRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(&*self.0, state);
    }
}

impl PartialEq for ComponentRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for ComponentRef {}


#[derive(Hash, Eq, PartialEq)]
struct Pin(ComponentRef, usize);

struct Push(usize, WireValue);

//struct Pin {
//    net: Option<Weak<Net>>,
//    handler: fn(WireValue) -> Option<Push>,
//}
//
//impl Pin {
//    fn new(handler: fn(WireValue) -> Option<Push>) -> Self {
//        Pin {net: None, handler: handler}
//    }
//}

struct Net(Vec<Pin>);

struct Netlist {
    nets: Vec<Net>,
}

impl Netlist {
    fn new() -> Self {
        Netlist {nets: vec![]}
    }
}

struct SimState {
    values: HashMap<Pin, WireValue>,
}

impl SimState {
    fn get(&self, pin: Pin) -> Option<WireValue> {
        self.values.get(&pin).map(|wv| *wv)
    }
}

struct ComponentState<'a>(Rc<dyn Component>, &'a SimState);

impl ComponentState<'_> {
    fn get(&self, pin_idx: usize) -> Option<WireValue> {
        self.1.get(Pin(ComponentRef(self.0.clone()), pin_idx))
    }
}

struct Simulator {
    netlist: Netlist,
}

impl Simulator {
    fn new(netlist: Netlist) -> Self {
        Simulator {netlist: netlist}
    }

    fn step(&mut self) {
        // TODO: work through our queue of updates. for each pin with an update,
        //       construct a componentstate and invoke each component's recv().
        //       add their pushes into the queue
    }
}

//struct Location {
//    x: u32,
//    y: u32,
//}
// x and y
struct Location(usize, usize);

//struct PinLoc(Pin, Location);
struct PinLoc(usize, Location);
//struct PinLoc {
//    //pin: Rc<Pin>,
//    pin: Pin,
//    loc: Location,
//}
//
//impl PinLoc {
//    fn new(pin: Rc<Pin>, loc: Location) -> Self {
//        PinLoc {pin: pin, loc: loc}
//    }
//}

trait Component {
    fn recv(&self, state: &ComponentState) -> Option<Push>;
    fn pin_locs(&self) -> Vec<PinLoc>;
}

struct NAND {
    loc: Location,
}
enum NANDPin {
    A,
    B,
    OUT,
}

impl NAND {
    fn new(loc: Location) -> Self {
        NAND {loc: loc}
        //let (x, y) = loc;
        //NAND {a:   PinLoc::new(Rc::new(Pin::new(update)), Location {x: x, y: y}),
        //      b:   PinLoc::new(Rc::new(Pin::new(update)), Location {x: x, y: y+2}),
        //      out: PinLoc::new(Rc::new(Pin::new(ignore)), Location {x: x+2, y: y+1})}
    }
}

impl Component for NAND {
    fn pin_locs(&self) -> Vec<PinLoc> {
        let Location(x, y) = self.loc;
        vec![PinLoc(NANDPin::A as usize,   Location(x,   y)),
             PinLoc(NANDPin::B as usize,   Location(x,   y+2)),
             PinLoc(NANDPin::OUT as usize, Location(x+2, y+1))]
        //vec![self.a, self.b, self.out]
    }

    fn recv(&self, state: &ComponentState) -> Option<Push> {
        match (state.get(NANDPin::A as usize), state.get(NANDPin::B as usize)) {
            (Some(a), Some(b)) => Some(Push(NANDPin::OUT as usize, !(a & b))),
            _ => None
        }
    }
}
