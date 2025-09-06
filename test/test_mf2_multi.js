const mf = new Intl.MessageFormat("en", "Hello {$name}. Welcome to {$city}.");
mf.format({
  name: "Saji",
  city: "Sapporo",
});
