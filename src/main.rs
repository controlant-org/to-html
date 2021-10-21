use anyhow::Result;
use std::{
  fmt::Write,
  io::{self, Read},
};

fn main() -> Result<()> {
  let mut buf = String::new();

  writeln!(
    buf,
    "<html>
<head>
<meta charset=\"utf-8\">
<style>
body {{
  background-color: #141414;
  color: white;
}}
.terminal {{
  overflow: auto;
  line-height: 120%;
}}
.terminal .hl {{
  color: #00ffff;
  font-weight: bold;
}}
.terminal .esc {{
  color: #d558f5;
  font-weight: bold;
}}
</style>
</head>
<body>
<pre class=\"terminal\">"
  )?;

  let mut input = String::new();
  let mut stdin = io::stdin();
  stdin.read_to_string(&mut input)?;

  let html = ansi_to_html::convert_escaped(&input)?;
  writeln!(buf, "{}", html)?;

  writeln!(buf, "</pre>\n</body>\n</html>")?;

  println!("{}", buf);

  Ok(())
}
