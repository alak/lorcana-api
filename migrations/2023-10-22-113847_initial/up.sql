-- Your SQL goes here
-- CREATE TABLE cards (
--   id VARCHAR PRIMARY KEY,
--   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
--   set_code VARCHAR NOT NULL,
--   number INTEGER NOT NULL,
--   set VARCHAR NOT NULL,
--   name VARCHAR NOT NULL,
--   color VARCHAR NOT NULL,
--   rarity VARCHAR NOT NULL,
--   card_market_handle VARCHAR NOT NULL
-- );

CREATE TABLE price_records (
  created_at TIMESTAMP NOT NULL PRIMARY KEY DEFAULT CURRENT_TIMESTAMP,
  card_id VARCHAR NOT NULL, 
  set_code VARCHAR NOT NULL,
  number INTEGER NOT NULL,
  min_price Float NOT NULL,
  avg_price Float NOT NULL,
  is_foil BOOLEAN NOT NULL DEFAULT FALSE,
  locale VARCHAR NOT NULL
);

CREATE TABLE prices (
  id VARCHAR NOT NULL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP REFERENCES price_records(created_at),
  card_id VARCHAR NOT NULL,
  price Float NOT NULL,
  seller_location VARCHAR NOT NULL,
  locale VARCHAR NOT NULL
);

-- CREATE TABLE localized_names (
--   id VARCHAR PRIMARY KEY,
--   card_id VARCHAR NOT NULL REFERENCES cards(id),
--   name VARCHAR NOT NULL,
--   locale VARCHAR NOT NULL
-- );

-- CREATE TABLE localized_colors (
--   color VARCHAR NOT NULL,
--   localized_color VARCHAR NOT NULL,
--   locale VARCHAR NOT NULL,
--   PRIMARY KEY(locale, color)
-- );

-- CREATE TABLE localized_rarities (
--   rarity VARCHAR NOT NULL,
--   localized_rarity VARCHAR NOT NULL,
--   locale VARCHAR NOT NULL,
--   PRIMARY KEY(locale, rarity)
-- );



-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('amber', 'amber', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('amber', 'ambre', 'fr');

-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('amethyst', 'amethyst', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('amethyst', 'améthyste', 'fr');

-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('emerald', 'emerald', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('emerald', 'émeraude', 'fr');

-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('ruby', 'ruby', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('ruby', 'rubis', 'fr');

-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('sapphire', 'sapphire', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('sapphire', 'saphir', 'fr');

-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('steel', 'steel', 'en');
-- INSERT INTO localized_colors(color, localized_color, locale) VALUES ('steel', 'acier', 'fr');


-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('common', 'common', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('common', 'commune', 'fr');

-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('uncommon', 'uncommon', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('uncommon', 'peu commune', 'fr');

-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('rare', 'rare', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('rare', 'rare', 'fr');

-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('super rare', 'super rare', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('super rare', 'super rare', 'fr');

-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('legendary', 'legendary', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('legendary', 'légendaire', 'fr');

-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('enchanted', 'enchanted', 'en');
-- INSERT INTO localized_rarities(rarity, localized_rarity, locale) VALUES ('enchanted', 'enchantée', 'fr');