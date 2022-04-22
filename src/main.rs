type Point = i32;
type PSCommand = String;
type PSLine = String;

struct PSPath {
    path: Vec<PSCommand>
}

impl PSPath {
    fn addCmd(&self, command: PSCommand) -> PSPath {
        let mut tmp = self.path.clone();
        tmp.push(command);
        return PSPath { path: tmp };
    }

    fn stroke(&self) -> PSLine {
        return format!("{}stroke\n", self.path.concat());
    }
}


fn newpath() -> PSPath {
    return PSPath { path: vec![String::from("newpath\n")] };
}


fn moveto(p1: Point, p2: Point) -> PSCommand {
    return format!("{} {} moveto\n", p1, p2);
}

fn lineto(p1: Point, p2: Point) -> PSCommand {
    return format!("{} {} lineto\n", p1, p2);
}

fn strokewidth(width: i32) -> PSCommand {
    return format!("{} setlinewidth\n", width)
}


fn main() {
    let stroke = newpath()
        .addCmd(moveto(100, 200))
        .addCmd(lineto(100, 500))
        .addCmd(strokewidth(2))
        .stroke();
    println!("{}", stroke);
}
