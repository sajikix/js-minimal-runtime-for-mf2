const mf = new Intl.MessageFormat("en", "Hello {$place}!");
mf.format({ place: "World" });
