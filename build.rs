fn main() {
  glib_build_tools::compile_resources(
      &["src"],
      "src/ui-resources.gresource.xml",
      "compiled.gresource",
  );
}