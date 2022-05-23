# TRace
A simple typing TUI racer with the main focus being the spanish language.
![TRace Demo](./readme-assets/TRaceDemo.gif)

## Features
- Statistics
- Written in rust so it's bLaZinGLy fAsT.
- By default the database only has texts in spanish but you only need to modify the databse.csv file adding anything you'll like and it should work. The chars recognized by the app are:

```rs
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z' // and upper case variants

'á', 'é', 'í', 'ó', 'ú', 'ä', 'ë', 'ï', 'ö', 'ü', 'ñ', // and upper case variants

'ç', ' ', ',', '.', ':', '"', '-', '@', ';', '<', '>', '+', '-', '_', '(', ')', '=', '*', '/', '¡', '!', '¿', '?', '#', '$', '%', '&', '°', '\'', '^', '~', '[', ']', '{', '}',

'1', '2', '3', '4', '5', '6', '7', '8', '9', '0'
```

## Roadmap
- The bar chart currently only shows the quantity that fits the screen. There should be a command to navigate through records.
- I plan to add local multiplayer but this will take a while.