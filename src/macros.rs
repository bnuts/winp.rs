macro_rules! getter {
    ($vname: ident, &mut $vtype: ty) => {
        pub fn $vname(&mut self) -> &mut $vtype {
            &mut self.$vname
        }
    };

    ($vname: ident, &$vtype: ty) => {
        pub fn $vname(&self) -> &$vtype {
            &self.$vname
        }
    };

    ($vname: ident, $vtype: ty) => {
        pub fn $vname(&self) -> $vtype {
            self.$vname
        }
    };
}
