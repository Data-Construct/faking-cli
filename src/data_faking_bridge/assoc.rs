// import { E_DF_NAMES_UUIDS, df_func_uuid, df_uuids } from "./gen/uuid";
// import { E_CATEGORIES, E_FAKING_RETURN_TYPES } from "./commons";

// pub enum E_DF {
//   // defaults - name
//   MALE_PREFIX_STANDARD,
//   FEMALE_PREFIX_STANDARD,
// }

// use std::collections::HashMap;

// lazy_static! {
//     pub static ref ASSOC: HashMap<&'static str, fn() -> String> = {
//         let mut m = HashMap::new();
//         // m.insert("A".to_string(), "foo");
//         // m.insert("B".to_string(), "bar");
//         // m.insert("C".to_string(), "baz");

//         m.insert(
//             "Bool",
//             data_faking::data::countries::canada::addresses::street_name
//            // "baz"
//         );

//         //     [E_DF_NAMES.BOOL]: {
// //       fn: faking.bool,
// //       return_type: E_FAKING_RETURN_TYPES.BOOLEAN,
// //     },
//         m
//     };
// }

pub enum FNVARI {
    Bool(fn() -> bool),
    I8(fn() -> i8),
    String(fn() -> String)
}

pub fn get_func_from_string(arg: &String) -> FNVARI {
    match arg.as_str() {
        "Bool" => FNVARI::Bool(data_faking::data::defaults::types::bool),
        "I8" => FNVARI::I8(data_faking::data::defaults::types::i8),
        "UUID V4" => FNVARI::String(data_faking::data::defaults::uuids::uuid_v4_wasm),
        _ => panic!("{} - no function found for this string", arg)
    }
}

// pub struct I_FN {
//   // eslint-disable-next-line @typescript-eslint/ban-types
//   // fn: Function, // () => string | number | boolean | null;
//   return_type: E_FAKING_RETURN_TYPES,
// }

// pub struct I_DFASSOC {
//   title: String,
//   ex_data: String,
//   func: Option<I_FN>,
// }

// // TODO(clearfeld): this is pretty nasty - think of something better later
// // export enum E_DF_NAMES {
// pub enum I_E_DF_NAMES {
//   // defaults - types
//   BOOL = "Bool",
//   I8 = "i8",
//   I16 = "i16",
//   I32 = "i32",
//   I64 = "i64",
//   ISIZE = "isize",
//   U8 = "u8",
//   U16 = "u16",
//   U32 = "u32",
//   U64 = "u64",
//   USIZE = "usize",
//   F32 = "f32",
//   F64 = "f64",

//   UUID_V1 = "UUID V1",
//   UUID_V3 = "UUID V3",
//   UUID_V4 = "UUID V4",
//   UUID_V5 = "UUID V5",
//   UUID_V6 = "UUID V6",
//   UUID_V7 = "UUID V7",
//   UUID_V8 = "UUID V8",
// }

// pub const df_func = {
//     // ...df_func_uuid,

//     // defaults - types
//     [E_DF_NAMES.BOOL]: {
//       fn: faking.bool,
//       return_type: E_FAKING_RETURN_TYPES.BOOLEAN,
//     },

//     [E_DF_NAMES.I8]: {
//       fn: faking.i8,
//       return_type: E_FAKING_RETURN_TYPES.I8,
//     },
//     [E_DF_NAMES.I16]: {
//       fn: faking.i16,
//       return_type: E_FAKING_RETURN_TYPES.I16,
//     },
//     [E_DF_NAMES.I32]: {
//       fn: faking.i32,
//       return_type: E_FAKING_RETURN_TYPES.I32,
//     },
//     [E_DF_NAMES.I64]: {
//       fn: faking.i64,
//       return_type: E_FAKING_RETURN_TYPES.I64,
//     },
//     [E_DF_NAMES.ISIZE]: {
//       fn: faking.isize,
//       return_type: E_FAKING_RETURN_TYPES.ISIZE,
//     },

//     [E_DF_NAMES.U8]: {
//       fn: faking.u8,
//       return_type: E_FAKING_RETURN_TYPES.U8,
//     },
//     [E_DF_NAMES.U16]: {
//       fn: faking.u16,
//       return_type: E_FAKING_RETURN_TYPES.U16,
//     },
//     [E_DF_NAMES.U32]: {
//       fn: faking.u32,
//       return_type: E_FAKING_RETURN_TYPES.U32,
//     },
//     [E_DF_NAMES.U64]: {
//       fn: faking.u64,
//       return_type: E_FAKING_RETURN_TYPES.U64,
//     },
//     [E_DF_NAMES.USIZE]: {
//       fn: faking.usize,
//       return_type: E_FAKING_RETURN_TYPES.USIZE,
//     },

//     [E_DF_NAMES.F32]: {
//       fn: faking.f32,
//       return_type: E_FAKING_RETURN_TYPES.F32,
//     },
//     [E_DF_NAMES.F64]: {
//       fn: faking.f64,
//       return_type: E_FAKING_RETURN_TYPES.F64,
//     },

//     [E_DF_NAMES_UUIDS.UUID_V1]: {
//       fn: faking.uuid_v1,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V3]: {
//       fn: faking.uuid_v3,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V4]: {
//       fn: faking.uuid_v4,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V5]: {
//       fn: faking.uuid_v5,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V6]: {
//       fn: faking.uuid_v6,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V7]: {
//       fn: faking.uuid_v7,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES_UUIDS.UUID_V8]: {
//       fn: faking.uuid_v8,
//       return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
// ]

//   //defaults - usernames
//   RANDOM_USERNAME = "Random Username",

//   //defaults - names
//   MALE_PREFIX_STANDARD = "Male prefix standard",
//   FEMALE_PREFIX_STANDARD = "Female prefix standard",
//   MALE_PREFIX = "Male prefix",
//   FEMALE_PREFIX = "Female prefix",
//   NEUTRAL_PREFIX = "Neutral prefix",
//   MALE_FIRST_NAME = "Male first name",
//   FEMALE_FIRST_NAME = "Female first name",
//   NEUTRAL_FIRST_NAME = "Neutral first name",
//   LAST_NAME = "Last name",

//   //defaults - phone numbers
//   PHONE_NUMBER_WITH_COUNTRY_CODE = "Phone number with country code",
//   PHONE_NUMBER = "Phone number",

//   //defaults - emails
//   STANDARD_GENERIC_EMAIL = "Standard generic email",
//   STANDARD_PUBLIC_EMAIL = "Standard public email",
//   STANDARD_PUBLIC_EMAIL_ALIAS = "Standard public email alias",
//   STANDARD_BUSINESS_EMAIL = "Standard business email",
//   STANDARD_GOVERNMENT_EMAIL = "Standard government email",
//   STANDARD_ACCOUNT_EMAIL = "Standard account email",

//   //defaults - crypto
//   MD5 = "md5",
//   SHA1 = "sha1",
//   SHA256 = "sha256",
//   SHA512 = "sha512",

//   //defaults - longitude latitude
//   LONGITUDE = "Longitude",
//   LATITUDE = "Latitude",
//   COORDINATES = "Coordinates",

//   //blockchain - bitcoin
//   MAINNET_ADDRESS = "Bitcoin Mainnet address",
//   TESTNET_ADDRESS = "Bitcoin Testnet address",
//   SIGNET_ADDRESS = "Bitcoin Signet address",
//   REGTEST_ADDRESS = "Bitcoin Regtest address",

//   //blockchain - ethereum
//   GENERATE_WALLET_ADDRESS = "Generate ethereum wallet address",

//   //religion - bible
//   BIBLE_FIGURE = "Biblical Figures",
//   BIBLE_LOCATION = "Biblical Locations",
//   BIBLE_QUOTE = "Biblical Quotes",

//   //media - elder scrolls
//   ES_CHARACTERS = "Elder Scrolls Characters",
//   ES_GAMES = "Elder Scrolls Games",
//   ES_LOCATIONS = "Elder Scrolls Locations",
//   ES_FACTIONS = "Elder Scrolls Factions",
//   ES_EVENTS = "Elder Scrolls Events",

//   //media - friends
//   FRIENDS_CHARACTERS = "Friends Characters",
//   FRIENDS_QUOTES = "Friends Quotes",
//   FRIENDS_LOCATIONS = "Friends Locations",

//   //media - games
//   GAMES_TITLES = "Game Titles",
//   GAMES_GENRES = "Game Genres",
//   GAMES_PLATFORMS = "Game Platforms",

//   //media - hp lovecraft
//   HP_DIETY = "HP Lovecraft Dieties",
//   HP_BOOKS = "HP Lovecraft Books",
//   HP_LOCATIONS = "HP Lovecraft Locations",
//   HP_ARTEFACTS = "HP Lovecraft Artefacts",

//   //media - kpop
//   KPOP_GROUP_FIRST_GEN = "Kpop Groups (first generation)",
//   KPOP_GROUP_SECOND_GEN = "Kpop Groups (second generation)",
//   KPOP_GROUP_THIRD_GEN = "Kpop Groups (third generation)",
//   KPOP_GIRL_GROUPS = "Kpop Girl Groups",
//   KPOP_BOY_BANDS = "Kpop Boy Bands",
//   KPOP_SOLOISTS = "Kpop Soloists",

//   //media - lord of the rings
//   LOTR_CHARACTERS = "Lord of the Rings Characters",
//   LOTR_QUOTES = "Lord of the Rings Quotes",
//   LOTR_LOCATIONS = "Lord of the Rings Locations",

//   //media - manga
//   MANGA_TITLES = "Manga Titles",
//   MANGA_GENRES = "Manga Genres",
//   MANGA_PLATFORMS = "Manga Platforms",

//   //media - mario
//   MARIO_CHARACTERS = "Mario Characters",
//   MARIO_GAMES = "Mario Games",
//   MARIO_LOCATIONS = "Mario Locations",

//   //media - minecraft
//   MINECRAFT_ACHIEVEMENT = "Minecraft Achievements",
//   MINECRAFT_BIOME = "Minecraft Biomes",
//   MINECRAFT_BLOCK = "Minecraft Blocks",
//   MINECRAFT_ENCHANTMENT = "Minecraft Enchantments",
//   MINECRAFT_GAME_MODE = "Minecraft Game Modes",
//   MINECRAFT_ITEM = "Minecraft Items",
//   MINECRAFT_MOB = "Minecraft Mobs",
//   MINECRAFT_STATUS_EFFECT = "Minecraft Status Effects",

//   //media - movies
//   MOVIES = "Movies",

//   //media - one piece
//   OP_CHARACTERS = "One Piece Characters",
//   OP_QUOTES = "One Piece Quotes",
//   OP_LOCATIONS = "One Piece Locations",
//   OP_ISLANDS = "One Piece Islands",
//   OP_SEAS = "One Piece Seas",

//   //media - pokemon
//   POKEMON_NAMES = "Pokemon Names",
//   POKEMON_MOVES = "Pokemon Moves",
//   POKEMON_LOCATIONS = "Pokemon Locations",

//   //media - seinfeld
//   SEINFELD_CHARACTERS = "Seinfeld Characters",
//   SEINFELD_QUOTES = "Seinfeld Quotes",
//   SEINFELD_LOCATIONS = "Seinfeld Businesses",

//   //media - silicon valley
//   SILICON_VALLEY_CHARACTERS = "Silicon Valley Characters",
//   SILICON_VALLEY_COMPANIES = "Silicon Valley Companies",
//   SILICON_VALLEY_QUOTES = "Silicon Valley Quotes",
//   SILICON_VALLEY_APPS = "Silicon Valley Apps",
//   SILICON_VALLEY_INVENTIONS = "Silicon Valley Inventions",
//   SILICON_VALLEY_MOTTOS = "Silicon Valley Mottos",
//   SILICON_VALLEY_URLS = "Silicon Valley URLs",
//   SILICON_VALLEY_EMAILS = "Silicon Valley Emails",

//   //media - simpsons
//   SIMPSONS_CHARACTERS = "Simpsons Characters",
//   SIMPSONS_QUOTES = "Simpsons Quotes",
//   SIMPSONS_LOCATIONS = "Simpsons Locations",
//   SIMPSONS_EPISODES = "Simpsons Episode Titles",

//   //media - spongebob
//   SPONGEBOB_CHARACTERS = "Spongebob Characters",
//   SPONGEBOB_QUOTES = "Spongebob Quotes",
//   SPONGEBOB_LOCATIONS = "Spongebob Locations",
//   SPONGEBOB_EPISODES = "Spongebob Episode Titles",

//   //media - starwars
//   STAR_WARS_CHARACTERS = "Star Wars Characters",
//   STAR_WARS_SQUADRON = "Star Wars Squadrons",
//   STAR_WARS_CALL_NUM = "Star Wars Call Numbers",
//   STAR_WARS_CALL_SIGNS = "Star Wars Call Signs",
//   STAR_WARS_DROIDS = "Star Wars Droids",
//   STAR_WARS_PLANETS = "Star Wars Planets",
//   STAR_WARS_SPECIES = "Star Wars Species",
//   STAR_WARS_VEHICLES = "Star Wars Vehicles",
//   STAR_WARS_WOOKIE_WORDS = "Star Wars Wookie Words",

//   //media - starwars (yoda)
//   STAR_WARS_YODA_QUOTES = "Yoda Quotes",

//   //media - studio ghibli
//   GHIBLI_CHARACTERS = "Studio Ghibli Characters",
//   GHIBLI_QUOTES = "Studio Ghibli Quotes",
//   GHIBLI_MOVIES = "Studio Ghibli Movies",

//   //media - the hobbit
//   HOBBIT_CHARACTERS = "The Hobbit Characters",
//   HOBBIT_QUOTES = "The Hobbit Quotes",
//   HOBBIT_LOCATIONS = "The Hobbit Locations",
//   HOBBIT_THORINS_COMPANY = "The Hobbit Thorin's Company Members",

//   //media - tolkien
//   TOLKIEN_CHARACTERS = "Tolkien Characters",
//   TOLKIEN_POEMS = "Tolkien Poems",
//   TOLKIEN_LOCATIONS = "Tolkien Locations",
//   TOLKIEN_RACES = "Tolkien Races",

//   //misc = adjective
//   POSITIVE_ADJECTIVES = "Positive Adjectives",
//   NEGATIVE_ADJECTIVES = "Negative Adjectives",

//   //misc - Agent Bot
//   BOT_AGENTS = "Bot Agents",

//   //misc - Ancients
//   GODS = "Gods",
//   PRIMORDIALS = "Primordials",
//   TITANS = "Titans",
//   HEROS = "Heros",

//   //misc - Animals
//   ANIMALS = "Animals",

//   //misc - Appliances
//   BRANDS = "Brands",
//   EQUIPMENTS = "Equipments",

//   //misc - Artists
//   ARTISTS = "artists",

//   //misc - barcodes
//   UPC_A = "UPC-A Barcode",
//   UPC_A_COMPOSITE_SYMBOLOGY = "UPC-A Barcode with Composite Symbology",
//   UPC_E = "UPC-E Barcode",
//   UPC_E_COMPOSITE_SYMBOLOGY = "UPC-E Barcode with Composite Symbology",
//   ISBN = "ISBN Barcode",
//   ISMN = "ISMN Barcode",
//   ISSN = "ISSN Barcode",

//   //misc - blood
//   BLOOD_TYPE = "Blood Types",
//   RH_FACTORY = "Blood: RH Factorys",
//   BLOOD_GROUP = "Blood: Groups",

//   //misc - books
//   BOOK_TITLE = "Book Titles",
//   BOOK_PUBLISHER = "Book Publishers",

//   //misc - business
//   BUSINESS_CATEGORY = "Business Categories",

//   //misc - chess
//   CHESS_PLAYER = "Chess Player",
//   CHESS_TOURNAMENTS = "Chess Tournaments",
//   CHESS_OPENING = "Chess Openings",
//   CHESS_TITLES = "Chess Titles",
//   CHESS_SQUARE_NAMES = "Chess Square Names",
//   CHESS_PIECE_NAMES = "Chess Piece Names",

//   //misc - codes
//   ASIN_CODES = "ASIN Codes",

//   //misc - commerce
//   DEPARTMENT = "Departments",
//   PRODUCT_ADJECTIVE = "Product Adjectives",
//   MATERIAL = "Materials",
//   PRODUCT = "Products",
//   PROMO_CODE_ADJECTIVE = "Promotion Code Adjectives",
//   PROMO_CODE_NOUN = "Promotion Code Nouns",
//   BRAND_COMMERCE = "Brand Commerces",
//   VENDOR = "Brand Vendors",
//   FULL_PROD_NAME = "Full Product Names",
//   FULL_PROMO_CODE = "Full Promotion Codes",
//   FULL_PROD_INFO = "Full Product Info",

//   //misc - compass
//   CARDINAL_WORDS = "Cardinal Words",
//   CARDINAL_ABBREVIATION = "Cardinal Abbreviations",
//   CARDINAL_AZIMUTH = "Cardinal Azimuths",
//   ORDINAL_WORDS = "Ordinal Words",
//   ORDINAL_ABBREVIATION = "Ordinal Abbreviations",
//   ORDINAL_AZIMUTH = "Ordinal Azimuths",
//   HALF_WIND_WORDS = "Half-wind Words",
//   HALF_WIND_ABBREVIATIONS = "Half-wind Abbreviations",
//   HALF_WIND_AZIMUTHS = "Half-wind Azimuths",
//   QUARTER_WIND_WORDS = "Quarter-wind Words",
//   QUARTER_WIND_ABBREVIATIONS = "Quarter-wind Abbreviations",
//   QUARTER_WIND_AZIMUTH = "Quarter-wind Azimuths",

//   //misc - Cryptocurrency
//   CRYPTO_NAMES = "Cryptocurrency Names",
//   CRYPTO_SYMBOLS = "Cryptocurrency Symbols",

//   //misc - Currencies
//   CURRENCY_NAMES = "Currency Names",
//   CURRENCY_SYMBOLS = "Currency Symbols",
//   CURRENCY_CODES = "Currency Codes",

//   //misc - Dates
//   DATE = "Dates",
//   DATE_DOW = "Day of Week",
//   DATE_FUTURE = "Future Date",
//   DATE_PAST = "Past Date",

//   //misc - Demographic
//   RACE = "Races",
//   SEX = "Sexes",
//   DEMONYM = "Denomyns",
//   EDUCATION_ATTAINMENT = "Educational Attainments",
//   MARITAL_STATUS = "Marital Statuses",

//   //misc - Device
//   MODEL_NAMES = "Model Names",
//   MANUFACTURER_NAMES = "Manufacturer Names",
//   SERIAL_CODES = "Serial Codes",

//   //misc - fashion
//   TERMS = "Fashion Terms",

//   //misc - food
//   ALLERGENS = "Allergens",
//   DISHES = "Dishes",
//   DESCRIPTIONS = "Food Descriptions",
//   INGREDIENTS = "Ingredients",
//   FRUITS = "Fruits",
//   VEGETABLES = "Vegetables",
//   SPICES = "Spices",
//   MEASUREMENTS = "Food Measurements",
//   MEASUREMENT_SIZES = "Measurement Sizes",
//   METRIC_MEASUREMENTS = "Food Measurements (Metric)",
//   SUSHIS = "Sushis",
//   ETHNIC_CATEGORIES = "Ethnic Categories (Food)",

//   //misc - Greek Philosophers
//   GREEK_PHILO_NAMES = "Greek Philosopher Names",
//   GREEK_PHILO_QUOTES = "Greek Philosopher Quotes",

//   //misc - Industry Segments
//   INDUSTRY = "Industries",
//   SUPER_SECTOR = "Super Sectors",
//   SECTORS = "Sectors",
//   SUB_SECTORS = "Subsectors",

//   //misc - IPV4
//   IPV4_ADDRESS = "IPV4 Address",
//   IPV4_ADDRESS_CIDR = "IPV4 Address With CIDR",
//   IPV4_ADDRESS_PUBLIC = "Public IPV4 Address",
//   IPV4_ADDRESS_PRIVATE = "Private IPV4 Address",

//   //misc - IPV6
//   IPV6_ADDRESS = "IPV6 Address",
//   IPV6_ADDRESS_CIDR = "IPV6 Address With CIDR",

//   //misc - jobs
//   JOB_FIELDS = "Employment - Fields",
//   JOB_SENIORITY = "Employment - Seniorities",
//   JOB_POSITIONS = "Employment - Positions",
//   JOB_KEY_SKILLS = "Employment - Key Skills",
//   JOB_TYPES = "Employment - Types",
//   JOB_ED_LEVELS = "Employment - Education Levels",

//   //misc - Lorem Ipsum
//   LI_WORD = "Lorem Ipsum Word",
//   LI_SENTENCE = "Lorem Ipsum Sentence",
//   LI_PARAGRAPH = "Lorem Ipsum Paragraph",
//   LI_PARAGRAPHS = "Lorem Ipsum Paragraphs",

//   //misc - MAC Address
//   MAC_ADDRESS = "MAC Addresses",

//   //misc - Marketing Buzzword
//   MARKETING_BUZZWORD = "Marketing Buzzwords",

//   //misc - Measurements
//   HEIGHT_MEASUREMENT = "Height Measurements",
//   LENGTH_MEASUREMENT = "Length Measurements",
//   VOLUME_MEASUREMENT = "Volume Measurements",
//   WEIGHT_MEASUREMENT = "Weight Measurements",
//   HEIGHT_MEASUREMENT_METRIC = "Height Measurements (Metric)",
//   LENGTH_MEASUREMENT_METRIC = "Length Measurements (Metric)",
//   VOLUME_MEASUREMENT_METRIC = "Volume Measurements (Metric)",
//   WEIGHT_MEASUREMENT_METRIC = "Weight Measurements (Metric)",

//   //misc - Military
//   ARMY_RANK = "Army Ranks",
//   MARINE_RANK = "Marine Ranks",
//   NAVY_RANK = "Navy Ranks",
//   COAST_GUARD_RANK = "Coast Guard Ranks",
//   AIR_FORCE_RANK = "Air Force Ranks",
//   SPACE_FORCE_RANK = "Space Force Ranks",
//   DOD_PAYGRADES = "Department of Defense Paygrades",

//   //misc - Programming Languages
//   PRGMR_NAME = "Programming Language Names",
//   PRGMR_CREATOR = "Programming Language Creators",

//   //misc - Quotes
//   MOTIVATIONAL_QUOTES = "Motivational Quotes",
//   PHILISOPHICAL_QUOTES = "Philisophical Quotes",
//   MOVIE_QUOTES = "Movie Quotes",

//   //misc - Relationship
//   FAMILIAL_DIRECT = "Familial Direct",
//   FAMILIAL_EXTENDED = "Familial Extended",
//   IN_LAW = "In-Laws",
//   SPOUSE = "Spouses",
//   PARENT = "Parents",
//   SIBLING = "Siblings",

//   //misc - Restaurant
//   NAME_PREFIXES = "Restaurant Name Prefixes",
//   NAME_SUFFIXES = "Restaurant Name Suffixes",
//   NAME_FULL = "Restaurant Full Names",
//   RESTAURANT_TYPE = "Restaurant Types",
//   RESTAURANT_DESCRIPTION = "Restaurant Descriptions",
//   RESTAURANT_REVIEWS = "Restaurant Reviews",
//   RESTAURANT_FULL = "Restaurants (All Details)",

//   //misc - Shakespeare
//   SHAKESPEARE_HAMLET = "Shakespeare - Hamlet Quotes",
//   SHAKESPEARE_KING_RICHARD = "Shakespeare - King Richard III Quotes",
//   SHAKESPEARE_AS_YOU_LIKE_IT = "Shakespeare - As You Like It Quotes",
//   SHAKESPEARE_RJ = "Shakespeare - Romeo and Juliet Quotes",

//   //misc - Space
//   PLANET = "Planets",
//   MOON = "Moons",
//   GALAXY = "Galaxies",
//   NEBULA = "Nebulas",
//   STAR_CLUSTER = "Star Clusters",
//   CONSTELLATION = "Constellations",
//   STAR = "Stars",
//   AGENCY = "Agencies",
//   AGENCY_ABV = "Agency Abbreviations",
//   NASA_SPACE_CRAFT = "NASA Space Crafts",
//   COMPANY = "Space Companies",
//   DISTANCE_MEASUREMENT = "Distant Measurements (Space)",
//   METEORITE = "Meteorites",
//   LAUNCH_VEHICLE = "Launch Vehicles",

//   //misc - Sports
//   SUMMER_OLYMPICS = "Summer Olympics Sports",
//   WINTER_OLYMPICS = "Winter Olympics Sports",
//   SUMMER_PARALYMPICS = "Summer Paralympics Sports",
//   WINTER_PARALYMPICS = "Winter Paralympics Sports",
//   ANCIENT_OLYMPICS = "Ancient Olympics Sports",
//   UNUSUAL_OLYMPICS = "Unusual Olympics Sports",

//   //misc - Stripe
//   STRIPE_CARD_DATA = "Card Data (Stripe)",
//   TOKEN_CARD_DATA = "Token Data (Stripe)",
//   STRIPE_INVALID_CARD_DATA = "Invalid Card Data (Stripe)",

//   //misc - Subscription
//   SUB_PLAN = "Subscription Plan",
//   SUB_STATUS = "Subscription Status",
//   SUB_METHOD = "Subscription Method",
//   SUB_TERM = "Subscription Term",
//   SUB_PAY_PLAN = "Subscription Payment Term",

//   //misc - Tea
//   BLACK_TEA = "Black Tea",
//   OOLONG_TEA = "Oolong Tea",
//   GREEN_TEA = "Green Tea",
//   WHITE_TEA = "White Tea",
//   HERBAL_TEA = "Herbal Tea",
//   TEA_TYPE = "Tea Type",

//   //datetime - date_naive
//   DATE_NAIVE = "Date Naive",
//   DATE_NAIVE_BEFORE_TODAY = "Date Naive Before Today",
//   DATE_NAIVE_AFTER_TODAY = "Date Naive After Today",

//   //datetime - datetime_naive
//   DATETIME_NAIVE = "Datetime Naive",
//   DATETIME_NAIVE_MILLI = "Datetime Naive Milisecond",
//   DATETIME_NAIVE_MICRO = "Datetime Naive Microsecond",
//   DATETIME_NAIVE_NANO = "Datetime Naive Nanosecond",

//   //datetime - month
//   MONTH = "Month",
//   MONTH_ABBR = "Month Abbreviated",
//   MONTH_ORDINAL = "Month Ordinal",
//   MONTH_ORDINAL0 = "Month Ordinal 0",

//   //datetime - time_naive
//   TIME_NAIVE = "Time Naive",
//   TIME_NAIVE_MILLI = "Time Naive Millisecond",
//   TIME_NAIVE_MICRO = "Time Naive Micro",
//   TIME_NAIVE_NANO = "Time Naive Nano",

//   //datetime - unix
//   UNIX_TS = "Unix Timestamp",

//   //datetime - weekday
//   WEEKDAY = "Weekday",
//   WEEKDAY_ABBR = "Weekday Abbreviated",

//   //datetime - year
//   YEAR = "Year",

//   //datetime - sql
//   SQL_TIME = "SQL Time",
//   SQL_SERVER_TIME = "SQL Server Time",
//   SQL_DATE = "SQL Date",
//   SQL_DATETIME = "SQL Datetime",
//   SQL_SERVER_DATETIME = "SQL Server DateTime",
//   SQL_SERVER_DATETIME2 = "SQL Server DateTime2",

//   // Canada
//   CANADA_STREET_SUFFIX = "Canada - Street Suffix",
//   CANADA_STREET_NAME = "Canada - Street Name",
//   // CANADA_CITY = "Canada - City",
//   // CANADA_PROVINCE_CODE = "Canada - Province Code",
//   CANADA_STREET_ADDRESS = "Canada - Street Address",
//   CANADA_SECONDARY_ADDRESS = "Canada - Secondary Address",
//   CANADA_FULL_ADDRESS = "Canada - Full Address",

//   // USA
//   USA_CITY_PREFIX = "USA - City Prefix",
//   USA_CITY_SUFFIX = "USA - City Suffix",
//   USA_STREET_SUFFIX = "USA - Street Suffix",
//   USA_STREET_NAME = "USA - Street Name",
//   USA_STATE = "USA - State",
//   USA_CITY = "USA - City",
//   // USA_ZIP_CODE = "USA - Zip Code",
//   USA_STREET_ADDRESS = "USA - Street Address",
//   USA_SECONDARY_ADDRESS = "USA - Secondary Address",
//   USA_FULL_ADDRESS = "USA - Full Address",
// }

// pub const E_DF_NAMES = { ...E_DF_NAMES_UUIDS, ...I_E_DF_NAMES };

// pub const df_func = {
//   // ...df_func_uuid,

//   // defaults - types
//   [E_DF_NAMES.BOOL]: {
//     fn: faking.bool,
//     return_type: E_FAKING_RETURN_TYPES.BOOLEAN,
//   },

//   [E_DF_NAMES.I8]: {
//     fn: faking.i8,
//     return_type: E_FAKING_RETURN_TYPES.I8,
//   },
//   [E_DF_NAMES.I16]: {
//     fn: faking.i16,
//     return_type: E_FAKING_RETURN_TYPES.I16,
//   },
//   [E_DF_NAMES.I32]: {
//     fn: faking.i32,
//     return_type: E_FAKING_RETURN_TYPES.I32,
//   },
//   [E_DF_NAMES.I64]: {
//     fn: faking.i64,
//     return_type: E_FAKING_RETURN_TYPES.I64,
//   },
//   [E_DF_NAMES.ISIZE]: {
//     fn: faking.isize,
//     return_type: E_FAKING_RETURN_TYPES.ISIZE,
//   },

//   [E_DF_NAMES.U8]: {
//     fn: faking.u8,
//     return_type: E_FAKING_RETURN_TYPES.U8,
//   },
//   [E_DF_NAMES.U16]: {
//     fn: faking.u16,
//     return_type: E_FAKING_RETURN_TYPES.U16,
//   },
//   [E_DF_NAMES.U32]: {
//     fn: faking.u32,
//     return_type: E_FAKING_RETURN_TYPES.U32,
//   },
//   [E_DF_NAMES.U64]: {
//     fn: faking.u64,
//     return_type: E_FAKING_RETURN_TYPES.U64,
//   },
//   [E_DF_NAMES.USIZE]: {
//     fn: faking.usize,
//     return_type: E_FAKING_RETURN_TYPES.USIZE,
//   },

//   [E_DF_NAMES.F32]: {
//     fn: faking.f32,
//     return_type: E_FAKING_RETURN_TYPES.F32,
//   },
//   [E_DF_NAMES.F64]: {
//     fn: faking.f64,
//     return_type: E_FAKING_RETURN_TYPES.F64,
//   },

//   [E_DF_NAMES_UUIDS.UUID_V1]: {
//     fn: faking.uuid_v1,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V3]: {
//     fn: faking.uuid_v3,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V4]: {
//     fn: faking.uuid_v4,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V5]: {
//     fn: faking.uuid_v5,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V6]: {
//     fn: faking.uuid_v6,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V7]: {
//     fn: faking.uuid_v7,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },
// [E_DF_NAMES_UUIDS.UUID_V8]: {
//     fn: faking.uuid_v8,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
// },

//   // defaults - names
//   [E_DF_NAMES.MALE_PREFIX_STANDARD]: {
//     fn: faking.male_prefix_standard,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FEMALE_PREFIX_STANDARD]: {
//     fn: faking.female_prefix_standard,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MALE_PREFIX]: {
//     fn: faking.male_prefix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FEMALE_PREFIX]: {
//     fn: faking.female_prefix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NEUTRAL_PREFIX]: {
//     fn: faking.neutral_prefix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MALE_FIRST_NAME]: {
//     fn: faking.male_first_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FEMALE_FIRST_NAME]: {
//     fn: faking.female_first_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NEUTRAL_FIRST_NAME]: {
//     fn: faking.neutral_first_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LAST_NAME]: {
//     fn: faking.last_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //defaults - usernames
//   [E_DF_NAMES.RANDOM_USERNAME]: {
//     fn: faking.random_username,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //defaults - phone numbers
//   [E_DF_NAMES.PHONE_NUMBER]: {
//     fn: faking.phone_number,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PHONE_NUMBER_WITH_COUNTRY_CODE]: {
//     fn: faking.phone_number_with_country_code,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //defaults - emails
//   [E_DF_NAMES.STANDARD_GENERIC_EMAIL]: {
//     fn: faking.standard_generic_email,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STANDARD_PUBLIC_EMAIL]: {
//     fn: faking.standard_public_email,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STANDARD_PUBLIC_EMAIL_ALIAS]: {
//     fn: faking.standard_public_email_alias,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STANDARD_BUSINESS_EMAIL]: {
//     fn: faking.standard_business_email,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STANDARD_GOVERNMENT_EMAIL]: {
//     fn: faking.standard_government_email,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //broken
//   /* [E_DF_NAMES.STANDARD_ACCOUNT_EMAIL]: {
//     fn: faking.standard_account_email,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   }, */
//   /* defaults - crypto */
//   [E_DF_NAMES.MD5]: {
//     fn: faking.md5,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHA1]: {
//     fn: faking.sha1,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHA256]: {
//     fn: faking.sha256,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHA512]: {
//     fn: faking.sha512,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   /* defaults - longitude latitude */
//   [E_DF_NAMES.LONGITUDE]: {
//     fn: faking.longitude,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LATITUDE]: {
//     fn: faking.latitude,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.COORDINATES]: {
//     fn: faking.coordinates,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //blockchain - bitcoin
//   [E_DF_NAMES.MAINNET_ADDRESS]: {
//     fn: faking.mainnet_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TESTNET_ADDRESS]: {
//     fn: faking.testnet_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SIGNET_ADDRESS]: {
//     fn: faking.signet_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.REGTEST_ADDRESS]: {
//     fn: faking.regtest_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //blockchain - ethereum
//   [E_DF_NAMES.GENERATE_WALLET_ADDRESS]: {
//     fn: faking.generate_wallet_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Religion - bible
//   [E_DF_NAMES.BIBLE_FIGURE]: {
//     fn: faking.bible_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.BIBLE_LOCATION]: {
//     fn: faking.bible_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.BIBLE_QUOTE]: {
//     fn: faking.bible_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Elder Scrolls
//   [E_DF_NAMES.ES_CHARACTERS]: {
//     fn: faking.es_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ES_GAMES]: {
//     fn: faking.es_games,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ES_LOCATIONS]: {
//     fn: faking.es_location,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ES_FACTIONS]: {
//     fn: faking.es_factions,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ES_EVENTS]: {
//     fn: faking.es_events,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Friends
//   [E_DF_NAMES.FRIENDS_CHARACTERS]: {
//     fn: faking.friends_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FRIENDS_LOCATIONS]: {
//     fn: faking.friends_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FRIENDS_QUOTES]: {
//     fn: faking.friends_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Games
//   [E_DF_NAMES.GAMES_TITLES]: {
//     fn: faking.games_title,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GAMES_GENRES]: {
//     fn: faking.games_genre,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GAMES_PLATFORMS]: {
//     fn: faking.games_platform,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - HP Lovecraft
//   [E_DF_NAMES.HP_DIETY]: {
//     fn: faking.lovecraft_deity,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HP_BOOKS]: {
//     fn: faking.lovecraft_book,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HP_LOCATIONS]: {
//     fn: faking.lovecraft_location,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HP_ARTEFACTS]: {
//     fn: faking.lovecraft_artefact,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Kpop
//   [E_DF_NAMES.KPOP_GROUP_FIRST_GEN]: {
//     fn: faking.kpop_group_first_generation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.KPOP_GROUP_SECOND_GEN]: {
//     fn: faking.kpop_group_second_generation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.KPOP_GROUP_THIRD_GEN]: {
//     fn: faking.kpop_group_third_generation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.KPOP_GIRL_GROUPS]: {
//     fn: faking.kpop_girl_groups,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.KPOP_BOY_BANDS]: {
//     fn: faking.kpop_boy_bands,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.KPOP_SOLOISTS]: {
//     fn: faking.kpop_soloists,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Lord of the Rings
//   [E_DF_NAMES.LOTR_CHARACTERS]: {
//     fn: faking.lord_of_the_rings_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LOTR_LOCATIONS]: {
//     fn: faking.lord_of_the_rings_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LOTR_QUOTES]: {
//     fn: faking.lord_of_the_rings_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Manga
//   [E_DF_NAMES.MANGA_TITLES]: {
//     fn: faking.manga_title,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MANGA_GENRES]: {
//     fn: faking.manga_genre,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MANGA_PLATFORMS]: {
//     fn: faking.manga_platform,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Mario
//   [E_DF_NAMES.MARIO_CHARACTERS]: {
//     fn: faking.mario_character,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MARIO_GAMES]: {
//     fn: faking.mario_game,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MARIO_LOCATIONS]: {
//     fn: faking.mario_location,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Minecraft
//   [E_DF_NAMES.MINECRAFT_ACHIEVEMENT]: {
//     fn: faking.minecraft_achievement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_BIOME]: {
//     fn: faking.minecraft_biome,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_BLOCK]: {
//     fn: faking.minecraft_block,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_ENCHANTMENT]: {
//     fn: faking.minecraft_enchantment,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_GAME_MODE]: {
//     fn: faking.minecraft_game_mode,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_ITEM]: {
//     fn: faking.minecraft_item,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_MOB]: {
//     fn: faking.minecraft_mob,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MINECRAFT_STATUS_EFFECT]: {
//     fn: faking.minecraft_status_effect,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Movies
//   [E_DF_NAMES.MOVIES]: {
//     fn: faking.movies,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - One Piece
//   [E_DF_NAMES.OP_CHARACTERS]: {
//     fn: faking.one_piece_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.OP_SEAS]: {
//     fn: faking.one_piece_seas,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.OP_ISLANDS]: {
//     fn: faking.one_piece_islands,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.OP_LOCATIONS]: {
//     fn: faking.one_piece_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.OP_QUOTES]: {
//     fn: faking.one_piece_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Pokemon
//   [E_DF_NAMES.POKEMON_NAMES]: {
//     fn: faking.pokemon_names,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.POKEMON_MOVES]: {
//     fn: faking.pokemon_moves,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.POKEMON_LOCATIONS]: {
//     fn: faking.pokemon_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Seinfeld
//   [E_DF_NAMES.SEINFELD_CHARACTERS]: {
//     fn: faking.seinfeld_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SEINFELD_QUOTES]: {
//     fn: faking.seinfeld_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SEINFELD_LOCATIONS]: {
//     fn: faking.seinfeld_businesses,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Silicon Valley
//   [E_DF_NAMES.SILICON_VALLEY_CHARACTERS]: {
//     fn: faking.silicon_valley_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_COMPANIES]: {
//     fn: faking.silicon_valley_companies,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_QUOTES]: {
//     fn: faking.silicon_valley_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_APPS]: {
//     fn: faking.silicon_valley_apps,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_INVENTIONS]: {
//     fn: faking.silicon_valley_inventions,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_MOTTOS]: {
//     fn: faking.silicon_valley_mottos,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_URLS]: {
//     fn: faking.silicon_valley_urls,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SILICON_VALLEY_EMAILS]: {
//     fn: faking.silicon_valley_emails,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Simpsons
//   [E_DF_NAMES.SIMPSONS_CHARACTERS]: {
//     fn: faking.simpsons_character,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SIMPSONS_LOCATIONS]: {
//     fn: faking.simpsons_location,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SIMPSONS_QUOTES]: {
//     fn: faking.simpsons_quote,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SIMPSONS_EPISODES]: {
//     fn: faking.simpsons_episode_title,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Spongebob
//   [E_DF_NAMES.SPONGEBOB_CHARACTERS]: {
//     fn: faking.spongebob_character,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPONGEBOB_LOCATIONS]: {
//     fn: faking.spongebob_location,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPONGEBOB_QUOTES]: {
//     fn: faking.spongebob_quote,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPONGEBOB_EPISODES]: {
//     fn: faking.spongebob_episode_title,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Star Wars
//   [E_DF_NAMES.STAR_WARS_CHARACTERS]: {
//     fn: faking.starwars_character,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_SQUADRON]: {
//     fn: faking.call_squadron,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_CALL_NUM]: {
//     fn: faking.call_number,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_CALL_SIGNS]: {
//     fn: faking.call_sign,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_DROIDS]: {
//     fn: faking.droid,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_PLANETS]: {
//     fn: faking.starwars_planet,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_SPECIES]: {
//     fn: faking.species,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_VEHICLES]: {
//     fn: faking.vehicle,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_WARS_WOOKIE_WORDS]: {
//     fn: faking.wookie_word,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Star Wars Yoda
//   [E_DF_NAMES.STAR_WARS_YODA_QUOTES]: {
//     fn: faking.yoda,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Studio Ghibli
//   [E_DF_NAMES.GHIBLI_CHARACTERS]: {
//     fn: faking.studio_ghibli_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GHIBLI_QUOTES]: {
//     fn: faking.studio_ghibli_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GHIBLI_MOVIES]: {
//     fn: faking.studio_ghibli_movies,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - The Hobbit
//   [E_DF_NAMES.HOBBIT_CHARACTERS]: {
//     fn: faking.the_hobbit_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HOBBIT_QUOTES]: {
//     fn: faking.the_hobbit_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HOBBIT_LOCATIONS]: {
//     fn: faking.the_hobbit_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HOBBIT_THORINS_COMPANY]: {
//     fn: faking.the_hobbit_thorins_company,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Media - Tolkien
//   [E_DF_NAMES.TOLKIEN_CHARACTERS]: {
//     fn: faking.tolkein_characters,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TOLKIEN_POEMS]: {
//     fn: faking.tolkein_poems,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TOLKIEN_LOCATIONS]: {
//     fn: faking.tolkein_locations,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TOLKIEN_RACES]: {
//     fn: faking.tolkein_races,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Adjective
//   [E_DF_NAMES.POSITIVE_ADJECTIVES]: {
//     fn: faking.positive_adjective,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NEGATIVE_ADJECTIVES]: {
//     fn: faking.negative_adjective,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Bot Agent
//   [E_DF_NAMES.BOT_AGENTS]: {
//     fn: faking.bot_agent,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Ancients
//   [E_DF_NAMES.GODS]: {
//     fn: faking.god,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PRIMORDIALS]: {
//     fn: faking.primordial,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TITANS]: {
//     fn: faking.titan,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HEROS]: {
//     fn: faking.hero,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Animals
//   [E_DF_NAMES.ANIMALS]: {
//     fn: faking.animal,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Appliances
//   [E_DF_NAMES.BRANDS]: {
//     fn: faking.brand,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.EQUIPMENTS]: {
//     fn: faking.equipment,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Artist
//   [E_DF_NAMES.ARTISTS]: {
//     fn: faking.artist,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Barcodes
//   [E_DF_NAMES.UPC_A]: {
//     fn: faking.upc_a,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.UPC_A_COMPOSITE_SYMBOLOGY]: {
//     fn: faking.upc_a_with_composite_symbology,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.UPC_E]: {
//     fn: faking.upc_e,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.UPC_E_COMPOSITE_SYMBOLOGY]: {
//     fn: faking.upc_e_with_composite_symbology,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ISBN]: {
//     fn: faking.isbn,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ISMN]: {
//     fn: faking.ismn,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ISSN]: {
//     fn: faking.issn,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Blood
//   [E_DF_NAMES.BLOOD_TYPE]: {
//     fn: faking.blood_type,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.BLOOD_GROUP]: {
//     fn: faking.group,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.RH_FACTORY]: {
//     fn: faking.rh_factory,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Books
//   [E_DF_NAMES.BOOK_TITLE]: {
//     fn: faking.title,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.BOOK_PUBLISHER]: {
//     fn: faking.publisher,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Business
//   [E_DF_NAMES.BUSINESS_CATEGORY]: {
//     fn: faking.category,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Chess
//   [E_DF_NAMES.CHESS_PLAYER]: {
//     fn: faking.chess_player,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CHESS_TOURNAMENTS]: {
//     fn: faking.chess_tournaments,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CHESS_OPENING]: {
//     fn: faking.chess_opening,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CHESS_TITLES]: {
//     fn: faking.chess_titles,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CHESS_SQUARE_NAMES]: {
//     fn: faking.chess_square_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CHESS_PIECE_NAMES]: {
//     fn: faking.chess_piece_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - ASIN
//   [E_DF_NAMES.ASIN_CODES]: {
//     fn: faking.asin,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Commerce
//   [E_DF_NAMES.DEPARTMENT]: {
//     fn: faking.department,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PRODUCT_ADJECTIVE]: {
//     fn: faking.product_adjective,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MATERIAL]: {
//     fn: faking.material,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PRODUCT]: {
//     fn: faking.product,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PROMO_CODE_ADJECTIVE]: {
//     fn: faking.promotion_code_adjective,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PROMO_CODE_NOUN]: {
//     fn: faking.promotion_code_noun,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.BRAND_COMMERCE]: {
//     fn: faking.brand_commerce,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.VENDOR]: {
//     fn: faking.vendor,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FULL_PROD_NAME]: {
//     fn: faking.full_product_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FULL_PROMO_CODE]: {
//     fn: faking.full_promotion_code,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FULL_PROD_INFO]: {
//     fn: faking.full_product_info,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Compass
//   [E_DF_NAMES.CARDINAL_WORDS]: {
//     fn: faking.cardinal_words,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CARDINAL_ABBREVIATION]: {
//     fn: faking.cardinal_abbreviation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CARDINAL_AZIMUTH]: {
//     fn: faking.cardinal_azimuth,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ORDINAL_WORDS]: {
//     fn: faking.ordinal_words,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ORDINAL_ABBREVIATION]: {
//     fn: faking.ordinal_abbreviation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ORDINAL_AZIMUTH]: {
//     fn: faking.ordinal_azimuth,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HALF_WIND_WORDS]: {
//     fn: faking.half_wind_words,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HALF_WIND_ABBREVIATIONS]: {
//     fn: faking.half_wind_abbreviation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HALF_WIND_AZIMUTHS]: {
//     fn: faking.half_wind_azimuth,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.QUARTER_WIND_WORDS]: {
//     fn: faking.quarter_wind_words,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.QUARTER_WIND_ABBREVIATIONS]: {
//     fn: faking.quarter_wind_abbreviation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.QUARTER_WIND_AZIMUTH]: {
//     fn: faking.quarter_wind_azimuth,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Cryptocurrency
//   [E_DF_NAMES.CRYPTO_NAMES]: {
//     fn: faking.cryptocurrency_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CRYPTO_SYMBOLS]: {
//     fn: faking.cryptocurrency_symbol,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Currencies
//   [E_DF_NAMES.CURRENCY_NAMES]: {
//     fn: faking.names,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CURRENCY_SYMBOLS]: {
//     fn: faking.symbols,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CURRENCY_CODES]: {
//     fn: faking.codes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Dates
//   //[E_DF_NAMES.DATE]: {
//   //	fn: faking.random_date,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},
//   //[E_DF_NAMES.DATE_DOW]: {
//   //	fn: faking.random_date_dow,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},
//   //[E_DF_NAMES.DATE_FUTURE]: {
//   //	fn: faking.random_date_future,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},
//   //[E_DF_NAMES.DATE_PAST]: {
//   //	fn: faking.random_date_past,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},

//   //Misc - Demographic
//   [E_DF_NAMES.RACE]: {
//     fn: faking.race,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SEX]: {
//     fn: faking.sex,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DEMONYM]: {
//     fn: faking.demonym,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.EDUCATION_ATTAINMENT]: {
//     fn: faking.educational_attainment,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MARITAL_STATUS]: {
//     fn: faking.martial_status,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Device
//   [E_DF_NAMES.MODEL_NAMES]: {
//     fn: faking.race,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MANUFACTURER_NAMES]: {
//     fn: faking.sex,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SERIAL_CODES]: {
//     fn: faking.demonym,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Fashion
//   [E_DF_NAMES.TERMS]: {
//     fn: faking.race,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Food
//   [E_DF_NAMES.ALLERGENS]: {
//     fn: faking.allergens,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DISHES]: {
//     fn: faking.dishes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DESCRIPTIONS]: {
//     fn: faking.descriptions,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.INGREDIENTS]: {
//     fn: faking.ingredients,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FRUITS]: {
//     fn: faking.fruits,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.VEGETABLES]: {
//     fn: faking.vegetables,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPICES]: {
//     fn: faking.spices,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MEASUREMENTS]: {
//     fn: faking.measurements,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MEASUREMENT_SIZES]: {
//     fn: faking.measurement_sizes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.METRIC_MEASUREMENTS]: {
//     fn: faking.metric_measurements,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUSHIS]: {
//     fn: faking.sushis,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ETHNIC_CATEGORIES]: {
//     fn: faking.ethnic_categories,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Greek Philosophers
//   [E_DF_NAMES.GREEK_PHILO_NAMES]: {
//     fn: faking.greek_philosopher_names,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GREEK_PHILO_QUOTES]: {
//     fn: faking.greek_philosopher_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Industry Segments
//   [E_DF_NAMES.INDUSTRY]: {
//     fn: faking.industry,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUPER_SECTOR]: {
//     fn: faking.super_sector,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SECTORS]: {
//     fn: faking.sector,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUB_SECTORS]: {
//     fn: faking.sub_sector,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - IPV4
//   [E_DF_NAMES.IPV4_ADDRESS]: {
//     fn: faking.ipv4_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.IPV4_ADDRESS_CIDR]: {
//     fn: faking.ipv4_address_with_cidr,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.IPV4_ADDRESS_PUBLIC]: {
//     fn: faking.ipv4_address_public,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.IPV4_ADDRESS_PRIVATE]: {
//     fn: faking.ipv4_address_private,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - IPV6
//   [E_DF_NAMES.IPV6_ADDRESS]: {
//     fn: faking.ipv6_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.IPV6_ADDRESS_CIDR]: {
//     fn: faking.ipv6_address_with_cidr,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Job
//   [E_DF_NAMES.JOB_FIELDS]: {
//     fn: faking.field,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.JOB_SENIORITY]: {
//     fn: faking.seniority,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.JOB_POSITIONS]: {
//     fn: faking.position,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.JOB_KEY_SKILLS]: {
//     fn: faking.key_skill,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.JOB_TYPES]: {
//     fn: faking.employment_type,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.JOB_ED_LEVELS]: {
//     fn: faking.education_level,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Lorem Ipsum
//   [E_DF_NAMES.LI_WORD]: {
//     fn: faking.lorem_ipsum_word,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LI_SENTENCE]: {
//     fn: faking.lorem_ipsum_sentence,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LI_PARAGRAPH]: {
//     fn: faking.lorem_ipsum_paragraph,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LI_PARAGRAPHS]: {
//     fn: faking.lorem_ipsum_paragraphs,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - MAC Address
//   [E_DF_NAMES.MAC_ADDRESS]: {
//     fn: faking.mac_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Marketing
//   [E_DF_NAMES.MARKETING_BUZZWORD]: {
//     fn: faking.marketing_buzzword,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Measurements
//   [E_DF_NAMES.HEIGHT_MEASUREMENT]: {
//     fn: faking.height_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LENGTH_MEASUREMENT]: {
//     fn: faking.length_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.VOLUME_MEASUREMENT]: {
//     fn: faking.volume_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WEIGHT_MEASUREMENT]: {
//     fn: faking.weight_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HEIGHT_MEASUREMENT_METRIC]: {
//     fn: faking.metric_height_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LENGTH_MEASUREMENT_METRIC]: {
//     fn: faking.metric_length_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.VOLUME_MEASUREMENT_METRIC]: {
//     fn: faking.metric_volume_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WEIGHT_MEASUREMENT_METRIC]: {
//     fn: faking.metric_weight_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Military
//   [E_DF_NAMES.ARMY_RANK]: {
//     fn: faking.army_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MARINE_RANK]: {
//     fn: faking.marine_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NAVY_RANK]: {
//     fn: faking.navy_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.COAST_GUARD_RANK]: {
//     fn: faking.coast_guard_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.AIR_FORCE_RANK]: {
//     fn: faking.air_force_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPACE_FORCE_RANK]: {
//     fn: faking.space_force_rank,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DOD_PAYGRADES]: {
//     fn: faking.dod_paygrade,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Programming Languages
//   [E_DF_NAMES.PRGMR_NAME]: {
//     fn: faking.programming_language_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PRGMR_CREATOR]: {
//     fn: faking.programming_language_creator,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Quotes
//   [E_DF_NAMES.MOTIVATIONAL_QUOTES]: {
//     fn: faking.motivational_quote,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PHILISOPHICAL_QUOTES]: {
//     fn: faking.philisophical_quote,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MOVIE_QUOTES]: {
//     fn: faking.movie_quote,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Relationship
//   [E_DF_NAMES.FAMILIAL_DIRECT]: {
//     fn: faking.familial_direct,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.FAMILIAL_EXTENDED]: {
//     fn: faking.familial_extended,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.IN_LAW]: {
//     fn: faking.in_law,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SPOUSE]: {
//     fn: faking.spouse,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.PARENT]: {
//     fn: faking.parent,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SIBLING]: {
//     fn: faking.sibling,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Restaurant
//   [E_DF_NAMES.NAME_PREFIXES]: {
//     fn: faking.name_prefix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NAME_SUFFIXES]: {
//     fn: faking.name_suffix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NAME_FULL]: {
//     fn: faking.restaurant_full_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.RESTAURANT_TYPE]: {
//     fn: faking.restaurant_type,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.RESTAURANT_DESCRIPTION]: {
//     fn: faking.restaurant_description,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.RESTAURANT_REVIEWS]: {
//     fn: faking.restaurant_review,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.RESTAURANT_FULL]: {
//     fn: faking.generate_full_restaurant,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Shakespeare
//   [E_DF_NAMES.SHAKESPEARE_HAMLET]: {
//     fn: faking.shakespeare_hamlet_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHAKESPEARE_AS_YOU_LIKE_IT]: {
//     fn: faking.shakespeare_as_you_like_it_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHAKESPEARE_KING_RICHARD]: {
//     fn: faking.shakespeare_king_richard_iii_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SHAKESPEARE_RJ]: {
//     fn: faking.shakespeare_romeo_and_juliet_quotes,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Space
//   [E_DF_NAMES.PLANET]: {
//     fn: faking.planet,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MOON]: {
//     fn: faking.moon,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GALAXY]: {
//     fn: faking.galaxy,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NEBULA]: {
//     fn: faking.nebula,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR_CLUSTER]: {
//     fn: faking.star_cluster,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CONSTELLATION]: {
//     fn: faking.constellation,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.STAR]: {
//     fn: faking.star,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.AGENCY]: {
//     fn: faking.agency,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.AGENCY_ABV]: {
//     fn: faking.agency_abv,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.NASA_SPACE_CRAFT]: {
//     fn: faking.nasa_space_craft,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.COMPANY]: {
//     fn: faking.company,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DISTANCE_MEASUREMENT]: {
//     fn: faking.distance_measurement,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.METEORITE]: {
//     fn: faking.meteorite,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.LAUNCH_VEHICLE]: {
//     fn: faking.launch_vehicle,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Sports
//   [E_DF_NAMES.SUMMER_OLYMPICS]: {
//     fn: faking.summer_olympic,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WINTER_OLYMPICS]: {
//     fn: faking.winter_olympic,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUMMER_PARALYMPICS]: {
//     fn: faking.summer_paralympic,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WINTER_PARALYMPICS]: {
//     fn: faking.winter_paralympics,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.ANCIENT_OLYMPICS]: {
//     fn: faking.ancient_olympic,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.UNUSUAL_OLYMPICS]: {
//     fn: faking.unusual,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Stripe
//   //[E_DF_NAMES.STRIPE_CARD_DATA]: {
//   //	fn: faking.valid_card,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},
//   //[E_DF_NAMES.TOKEN_CARD_DATA]: {
//   //	fn: faking.valid_card_vendor,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},
//   //[E_DF_NAMES.STRIPE_INVALID_CARD_DATA]: {
//   //	fn: faking.valid_card_number,
//   //	return_type: E_FAKING_RETURN_TYPES.STRING,
//   //},

//   //Misc - Subscription
//   [E_DF_NAMES.SUB_PLAN]: {
//     fn: faking.plan,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUB_STATUS]: {
//     fn: faking.status,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUB_METHOD]: {
//     fn: faking.payment_method,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUB_TERM]: {
//     fn: faking.subscription_term,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SUB_PAY_PLAN]: {
//     fn: faking.payment_term,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //Misc - Tea
//   [E_DF_NAMES.BLACK_TEA]: {
//     fn: faking.black_tea,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.OOLONG_TEA]: {
//     fn: faking.oolong_tea,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.GREEN_TEA]: {
//     fn: faking.green_tea,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WHITE_TEA]: {
//     fn: faking.white_tea,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.HERBAL_TEA]: {
//     fn: faking.herbal_tea,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TEA_TYPE]: {
//     fn: faking.tea_type,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - date_naive
//   [E_DF_NAMES.DATE_NAIVE]: {
//     fn: faking.date_naive,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DATE_NAIVE_AFTER_TODAY]: {
//     fn: faking.date_naive_after_today,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DATE_NAIVE_BEFORE_TODAY]: {
//     fn: faking.date_naive_before_today,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - datetime_naive
//   [E_DF_NAMES.DATETIME_NAIVE]: {
//     fn: faking.datetime_naive,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DATETIME_NAIVE_MILLI]: {
//     fn: faking.datetime_naive_milli,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DATETIME_NAIVE_MICRO]: {
//     fn: faking.datetime_naive_micro,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.DATETIME_NAIVE_NANO]: {
//     fn: faking.datetime_naive_nano,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - month
//   [E_DF_NAMES.MONTH]: {
//     fn: faking.month,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MONTH_ABBR]: {
//     fn: faking.month_abbr,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MONTH_ORDINAL]: {
//     fn: faking.month_ordinal,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.MONTH_ORDINAL0]: {
//     fn: faking.month_ordinal0,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - time_naive
//   [E_DF_NAMES.TIME_NAIVE]: {
//     fn: faking.time_naive,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TIME_NAIVE_MICRO]: {
//     fn: faking.time_naive_micro,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TIME_NAIVE_MILLI]: {
//     fn: faking.time_naive_milli,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.TIME_NAIVE_NANO]: {
//     fn: faking.time_naive_nano,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - unix
//   [E_DF_NAMES.UNIX_TS]: {
//     fn: faking.unix_ts,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - weekday
//   [E_DF_NAMES.WEEKDAY]: {
//     fn: faking.weekday,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.WEEKDAY_ABBR]: {
//     fn: faking.weekday_abbr,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - year
//   [E_DF_NAMES.YEAR]: {
//     fn: faking.year,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   //datetime - sql
//   [E_DF_NAMES.SQL_TIME]: {
//     fn: faking.sql_time,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SQL_SERVER_TIME]: {
//     fn: faking.sql_server_time,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SQL_DATE]: {
//     fn: faking.sql_date,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SQL_DATETIME]: {
//     fn: faking.sql_datetime,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SQL_SERVER_DATETIME]: {
//     fn: faking.sql_server_datetime,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.SQL_SERVER_DATETIME2]: {
//     fn: faking.sql_server_datetime2,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   // Canada
//   [E_DF_NAMES.CANADA_STREET_SUFFIX]: {
//     fn: faking.canada_street_suffix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CANADA_STREET_NAME]: {
//     fn: faking.canada_street_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   // [E_DF_NAMES.CANADA_CITY]: {
//   //   fn: faking.canada_city,
//   //   return_type: E_FAKING_RETURN_TYPES.STRING,
//   // },
//   // [E_DF_NAMES.CANADA_PROVINCE_CODE]: {
//   //   fn: faking.canada_province,
//   //   return_type: E_FAKING_RETURN_TYPES.STRING,
//   // },
//   [E_DF_NAMES.CANADA_STREET_ADDRESS]: {
//     fn: faking.canada_street_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CANADA_SECONDARY_ADDRESS]: {
//     fn: faking.canada_secondary_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.CANADA_FULL_ADDRESS]: {
//     fn: faking.canada_full_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },

//   // USA
//   [E_DF_NAMES.USA_CITY_PREFIX]: {
//     fn: faking.usa_city_prefix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_CITY_SUFFIX]: {
//     fn: faking.usa_city_suffix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_STREET_SUFFIX]: {
//     fn: faking.usa_street_suffix,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_STREET_NAME]: {
//     fn: faking.usa_street_name,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_STATE]: {
//     fn: faking.usa_state,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_CITY]: {
//     fn: faking.usa_city,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   // [E_DF_NAMES.USA_ZIP_CODE]: {
//   //   fn: faking.usa_zip_code,
//   //   return_type: E_FAKING_RETURN_TYPES.STRING,
//   // },
//   [E_DF_NAMES.USA_STREET_ADDRESS]: {
//     fn: faking.usa_street_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_SECONDARY_ADDRESS]: {
//     fn: faking.usa_secondary_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//   [E_DF_NAMES.USA_FULL_ADDRESS]: {
//     fn: faking.usa_full_address,
//     return_type: E_FAKING_RETURN_TYPES.STRING,
//   },
//};
