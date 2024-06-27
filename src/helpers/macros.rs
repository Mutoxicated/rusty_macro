#[macro_export]
macro_rules! csstruct {
    ($([ $( $derive:ident ),* ])? $name:ident; $( $v:vis $field:ident:$field_type:ty $(| $setter:ident)? $(; $getter:ident )?),* ) => {
        $( #[derive( $($derive),* )] )?
        pub struct $name {
            $( $v $field:$field_type ),*
        }

        impl $name {
            $( 
                $(pub fn $setter(&mut self, val:$field_type) {
                    self.$field = val
                })?

                $(pub fn $getter(&self) -> &$field_type {
                    &self.$field
                })?
            )*
        }
    };
}

#[macro_export]
macro_rules! construct {
    ($name:ident; $( { $($t:tt)*} )? $($field:ident = $val:expr),*) => {
        impl $name {
            pub fn new() -> Self {
                $( $($t)* )?
                Self {
                    $( $field:$val ),*
                }
            }
        }
    };
    ($name:ident; $( { $($t:tt)*} )? $($field:ident = $val:expr),*; $var:ident { $( $t2:tt )* }) => {
        impl $name {
            pub fn new() -> Self {
                $( $($t)* )?
                let mut $var = Self {
                    $( $field:$val ),*
                };
                $( $t2 )*
                $var
            }
        }
    };
}

#[macro_export]
macro_rules! egui_options {
    ($var:ident) => {
        let $var = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_min_inner_size([700.0,700.0])
                .with_resizable(false)
                .with_minimize_button(false)
                .with_maximize_button(false),
            ..Default::default()
        };
    };
}