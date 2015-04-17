#[macro_export]
macro_rules! tcod_print {
    // ABW
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr), 
     Bg($bg: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );

    // AWB
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr),
     Wrap($width: expr, $height: expr), Bg($bg: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );
    
    // BAW
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), 
     Align($alignment: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );
    
    // BWA
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), 
     Wrap($width: expr, $height: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );
     
    // WAB
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), 
     Align($alignment: expr), Bg($bg: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );
    
    // WBA
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), 
     Bg($bg: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*).as_ref());
    );
    
    // AB
    ($con: expr, At($x: expr, $y: expr), Align($bg: expr), Bg($alignment: expr), $($arg: tt)*) => (
        $con.print_ex($x, $y, $bg, $alignment, format!($($arg)*).as_ref());
    );    
     
    // AW
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_rect_ex($x, $y, $width, $height, bg, $alignment, format!($($arg)*).as_ref());
        }
    );
   
    // BA
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_ex($x, $y, $bg, $alignment, format!($($arg)*).as_ref());
    );
     
    // BW
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_rect_ex($x, $y, $width, $height, $bg, alignment, format!($($arg)*).as_ref());
        }
    );
    
    // WA
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), Align($alignment: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_rect_ex($x, $y, $width, $height, bg, $alignment, format!($($arg)*).as_ref());
        }
    );
    
    // WB
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), Bg($bg: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_rect_ex($x, $y, $width, $height, $bg, alignment, format!($($arg)*).as_ref());
        }
    );
    
    // A
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_ex($x, $y, bg, $alignment, format!($($arg)*).as_ref());
        }
    );
    
    // B
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_ex($x, $y, $bg, alignment, format!($($arg)*).as_ref());
        }
    );

    // W
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect($x, $y, $width, $height, format!($($arg)*).as_ref());
    );
    
    // None
    ($con: expr, At($x: expr, $y: expr), $($arg: tt)*) => (
        $con.print($x, $y, format!($($arg)*).as_ref());
    );
}
