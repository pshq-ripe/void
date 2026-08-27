# Podsumowanie wprowadzonych zmian w kliencie IRC Void (System Motywów)

Niniejszy dokument opisuje wszystkie zmiany wprowadzone w silniku motywów kolorystycznych oraz obsłudze stylów w kliencie IRC Void.

---

## 1. Zmiany w kodzie źródłowym Rust (`src/`)

### `src/app.rs`
- **Rozszerzenie struktury `ThemeColors`**:
  - Dodano pola metadanych: `desc` (opis motywu) oraz `is_dark` (flaga ciemnego/jasnego motywu).
  - Dodano brakujące kolory elementów UI: `input_bg`, `input_prompt_fg`, `scroll_indicator_fg`, `scroll_indicator_bg`, `nick_list_header`, `msg_url`.
  - Dodano pole `nick_colors: Vec<Color>` dla dynamicznej palety kolorów nicków na czacie.
  - Zaktualizowano domyślną implementację `ThemeColors::default()` na zbalansowaną, wysoce czytelną paletę w stylu Catppuccin Mocha.
- **Kompleksowy silnik parsowania kolorów (`apply_theme`)**:
  - Dodano pełne wsparcie dla formatów szesnastkowych: `#RGB`, `#RRGGBB`, `0xRRGGBB` mapowanych bezpośrednio na `Color::Rgb(r, g, b)`.
  - Dodano wsparcie dla formatu `rgb(r, g, b)` oraz tabel Lua `{r, g, b}` / `{r=..., g=..., b=...}`.
  - Dodano wsparcie dla 256 kolorów ANSI (`0..255`, `idx:N`, `ansi:N`).
  - Rozszerzono mapowanie nazw kolorów (np. `darkred`, `darkgreen`, `darkblue`, `navy`, `darkmagenta`, `darkcyan`, `darkyellow`, `orange`, `peach`, `violet`, `lavender`, `gold`, `brown`).
  - Naprawiono błąd fallbacku: nieznane lub brakujące wartości nie powodują już ustawienia białego tła (`Color::White`), lecz zachowują pierwotną wartość lub `Color::Reset`.

### `src/ui/renderer.rs`
- **Zintegrowanie motywu z renderowaniem czatu**:
  - `nick_color`: Funkcja hashowania nicków korzysta z dedykowanej palety `theme.nick_colors`.
  - `highlight_urls` & `parse_irc_formatting`: Podświetlanie URL używa koloru `theme.msg_url`.
  - Przedrostki rang (`~`, `&`, `@`, `%`, `+`) na czacie używają kolorów `theme.nick_founder`, `theme.nick_admin`, `theme.nick_op`, `theme.nick_halfop`, `theme.nick_voice`.
  - Timestampy używają koloru `theme.timestamp` zamiast na sztywno przypisanego ciemnoszarego.
  - Indykator przewijania nowych wiadomości (`[N new]`) używa stylów `theme.scroll_indicator_fg` oraz `theme.scroll_indicator_bg`.
  - Dodano opcjonalne tło bufora czatu (`theme.chat_bg`).
- **Lista użytkowników i pasek wejścia**:
  - Nagłówki grup w liście nicków (`Ops`, `Voices`, `Users`) używają stylu `theme.nick_list_header`.
  - Obramowanie listy nicków i paska wejścia korzysta z `theme.border`.
  - Znak zachęty paska wejścia (`INPUT_PROMPT`) używa koloru `theme.input_prompt_fg`.

### `src/format.rs`
- **Zharmonizowanie tokenów status baru z motywem**:
  - Znaczniki `%T` (czas), `%N` (nick), `%C` / `%R` (kanał), `%S` / `%H` (serwer/host), `%W` (okno), `%A` (away), `%M` / `%F` (tryby), `%#` / `%U` (liczba użytkowników), `%*` (nieprzeczytane), `%@` (status opa), `%+` (status voice), `%Q` (query) zostały podpięte pod odpowiednie właściwości `ThemeColors` w celu zapewnienia czytelności na dowolnym kolorze tła paska stanu.

### `src/main.rs`
- Dodano automatyczne ładowanie motywu na starcie aplikacji z konfiguracji `config.theme` lub z aktywnego motywu tabeli `void_themes`.

---

## 2. Zmiany w systemie motywów Lua (`modules/themes/`)

### `modules/themes/init.lua`
- Przebudowano moduł zarządzania motywami:
  - `void_themes.register(name, theme)`: Rejestracja z metadanymi i flagą `is_dark`.
  - `void_themes.apply(name)`: Bezpieczne stosowanie motywu (case-insensitive) z komunikatem potwierdzającym.
  - `void_themes.list()`: Czytelne zestawienie motywów z podziałem na sekcje `[Dark Themes]` i `[Light Themes]` oraz indykatorem aktywnego motywu `[*ACTIVE*]`.
  - `void_themes.info(name)`: Wyświetlanie szczegółowych parametrów danego motywu.
  - `void_themes.random()`: Losowy wybór motywu.
  - Komenda `/theme`: Obsługa parametrów `/theme`, `/theme list`, `/theme info <name>`, `/theme random`, `/theme <name>`.

### Uaktualnione istniejące motywy
- **`modules/themes/catppuccin.lua`** — Autentyczna paleta Catppuccin Mocha z pastelowymi odcieniami (Lavender, Mauve, Sapphire, Teal).
- **`modules/themes/dracula.lua`** — Kultowy motyw Dracula z neonowymi akcentami (Purple, Pink, Cyan, Green).
- **`modules/themes/nord.lua`** — Arktyczna paleta Nord (Polar Night, Frost, Aurora).
- **`modules/themes/gruvbox.lua`** — Ciepły retro schemat Gruvbox Dark z ziemistymi barwami.
- **`modules/themes/solarized.lua`** — Precyzyjna ciemna paleta Solarized Dark Ethana Schoonovera.
- **`modules/themes/tokyonight.lua`** — Nocny motyw Tokyo Night inspirowany neonami Akihabary.
- **`modules/themes/matrix.lua`** — Fosforowy zielony terminal hakerski na czarnym tle CRT.

### Nowe motywy
- **`modules/themes/catppuccin_latte.lua`** — Jasny pastelowy wariant Catppuccin Latte.
- **`modules/themes/gruvbox_light.lua`** — Jasny pergaminowy retro wariant Gruvbox Light.
- **`modules/themes/solarized_light.lua`** — Niskokontrastowy jasny wariant Solarized Light.
- **`modules/themes/cyberpunk.lua`** — Styl 80s outrun / synthwave z neonowym różem, cyjanem i żółcią.
- **`modules/themes/monokai.lua`** — Wyrazista paleta programistyczna Monokai Pro.
- **`modules/themes/onedark.lua`** — Zbalansowana, ikoniczna ciemna paleta edytora Atom.
- **`modules/themes/rosepine.lua`** — Estetyka Soho ze stonowanym różem, sosną i złotem.
- **`modules/themes/irssi.lua`** — Klasyczny styl IRC z niebieskim paskiem stanu i zielonymi akcentami.
- **`modules/themes/bitchx.lua`** — Retro styl klienta BitchX z lat 90. z wyrazistą czerwienią i cyjanem.

### `modules/init.lua`
- Zaktualizowano listę ładowanych modułów o wszystkie 16 motywów.

---

## 3. Testy i dokumentacja

### `tests/lua_integration.rs`
- Dodano zestaw testów automatycznych weryfikujących poprawność rejestracji, parsowania kolorów i aplikowania wszystkich 16 motywów w strukturze `App`.

### `README.md` oraz `DOCS.md`
- Zaktualizowano sekcje dotyczące motywów, liczby modułów oraz dokumentację komendy `/theme` (dodano opis nowych subkomend `info` oraz `random`).
