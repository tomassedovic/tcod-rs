#[macro_export]
macro_rules! tcod_print {
    // ABW
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr),
     Bg($bg: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // AWB
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr),
     Wrap($width: expr, $height: expr), Bg($bg: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // BAW
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr),
     Align($alignment: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // BWA
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr),
     Wrap($width: expr, $height: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // WAB
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr),
     Align($alignment: expr), Bg($bg: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // WBA
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr),
     Bg($bg: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_rect_ex($x, $y, $width, $height, $bg, $alignment, format!($($arg)*));
    );

    // AB
    ($con: expr, At($x: expr, $y: expr), Align($bg: expr), Bg($alignment: expr), $($arg: tt)*) => (
        $con.print_ex($x, $y, $bg, $alignment, format!($($arg)*));
    );

    // AW
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_rect_ex($x, $y, $width, $height, bg, $alignment, format!($($arg)*));
        }
    );

    // BA
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), Align($alignment: expr), $($arg: tt)*) => (
        $con.print_ex($x, $y, $bg, $alignment, format!($($arg)*));
    );

    // BW
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_rect_ex($x, $y, $width, $height, $bg, alignment, format!($($arg)*));
        }
    );

    // WA
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), Align($alignment: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_rect_ex($x, $y, $width, $height, bg, $alignment, format!($($arg)*));
        }
    );

    // WB
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), Bg($bg: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_rect_ex($x, $y, $width, $height, $bg, alignment, format!($($arg)*));
        }
    );

    // A
    ($con: expr, At($x: expr, $y: expr), Align($alignment: expr), $($arg: tt)*) => (
        {
            let bg = $con.get_background_flag();
            $con.print_ex($x, $y, bg, $alignment, format!($($arg)*));
        }
    );

    // B
    ($con: expr, At($x: expr, $y: expr), Bg($bg: expr), $($arg: tt)*) => (
        {
            let alignment = $con.get_alignment();
            $con.print_ex($x, $y, $bg, alignment, format!($($arg)*));
        }
    );

    // W
    ($con: expr, At($x: expr, $y: expr), Wrap($width: expr, $height: expr), $($arg: tt)*) => (
        $con.print_rect($x, $y, $width, $height, format!($($arg)*));
    );

    // None
    ($con: expr, At($x: expr, $y: expr), $($arg: tt)*) => (
        $con.print($x, $y, format!($($arg)*));
    );
}
