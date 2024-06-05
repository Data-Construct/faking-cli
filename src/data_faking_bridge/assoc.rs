use std::process;

use data_faking::data::api::stripe::card::{CardData, InvalidCardData, TokenData};

pub enum FNVARI {
    Bool(fn() -> bool),

    I8(fn() -> i8),
    I16(fn() -> i16),
    I32(fn() -> i32),
    I64(fn() -> i64),
    Isize(fn() -> isize),

    U8(fn() -> u8),
    U16(fn() -> u16),
    U32(fn() -> u32),
    U64(fn() -> u64),
    Usize(fn() -> usize),

    F32(fn() -> f32),
    F64(fn() -> f64),

    String(fn() -> String),

    CardData(fn() -> CardData),
    TokenData(fn() -> TokenData),
    InvalidCardData(fn() -> InvalidCardData),
}

pub fn get_func_from_string(arg: &String) -> FNVARI {
    match arg.as_str() {
        "Bool" => FNVARI::Bool(data_faking::data::defaults::types::bool),

        "ISize" => FNVARI::Isize(data_faking::data::defaults::types::isize),
        "I8" => FNVARI::I8(data_faking::data::defaults::types::i8),
        "I16" => FNVARI::I16(data_faking::data::defaults::types::i16),
        "I32" => FNVARI::I32(data_faking::data::defaults::types::i32),
        "I64" => FNVARI::I64(data_faking::data::defaults::types::i64),

        "U8" => FNVARI::U8(data_faking::data::defaults::types::u8),
        "U16" => FNVARI::U16(data_faking::data::defaults::types::u16),
        "U32" => FNVARI::U32(data_faking::data::defaults::types::u32),
        "U64" => FNVARI::U64(data_faking::data::defaults::types::u64),
        "USize" => FNVARI::Usize(data_faking::data::defaults::types::usize),

        "F32" => FNVARI::F32(data_faking::data::defaults::types::f32),
        "F64" => FNVARI::F64(data_faking::data::defaults::types::f64),

        "Random Username" => FNVARI::String(data_faking::locales::en::person::usernames::random_username),
        "Male prefix standard" => FNVARI::String(data_faking::locales::en::person::name::male_prefix_standard),
        "Female prefix standard" => FNVARI::String(data_faking::locales::en::person::name::female_prefix_standard),
        "Male prefix" => FNVARI::String(data_faking::locales::en::person::name::male_prefix),
        "Female prefix" => FNVARI::String(data_faking::locales::en::person::name::female_prefix),
        "Neutral prefix" => FNVARI::String(data_faking::locales::en::person::name::neutral_prefix),
        "Male first name" => FNVARI::String(data_faking::locales::en::person::name::male_first_name),
        "Female first name" => FNVARI::String(data_faking::locales::en::person::name::female_first_name),
        "Neutral first name" => FNVARI::String(data_faking::locales::en::person::name::neutral_first_name),
        "Last name" => FNVARI::String(data_faking::locales::en::person::name::last_name),

        //defaults - phone numbers
        "Phone number with country code" => FNVARI::String(data_faking::locales::en::person::phone_numbers::phone_number_with_country_code),
        "Phone number" => FNVARI::String(data_faking::locales::en::person::phone_numbers::phone_number),

        //defaults - emails
        "Standard generic email" => FNVARI::String(data_faking::locales::en::person::emails::standard_generic_email),
        "Standard public email" => FNVARI::String(data_faking::locales::en::person::emails::standard_public_email),
        "Standard public email alias" => FNVARI::String(data_faking::locales::en::person::emails::standard_public_email_alias),
        "Standard business email" => FNVARI::String(data_faking::locales::en::person::emails::standard_business_email),
        "Standard government email" => FNVARI::String(data_faking::locales::en::person::emails::standard_government_email),

        // TODO: This function requires two arguments
        // "Standard account email" => FNVARI::String(data_faking::locales::en::person::emails::standard_account_email),

        //defaults - crypto
        "md5" => FNVARI::String(data_faking::data::defaults::crypto::md5),
        "sha1" => FNVARI::String(data_faking::data::defaults::crypto::sha1),
        "sha256" => FNVARI::String(data_faking::data::defaults::crypto::sha256),
        "sha512" => FNVARI::String(data_faking::data::defaults::crypto::sha512),

        //defaults - longitude latitude
        "Longitude" => FNVARI::String(data_faking::data::defaults::longitude_latitude::longitude),
        "Latitude" => FNVARI::String(data_faking::data::defaults::longitude_latitude::latitude),
        "Coordinates" => FNVARI::String(data_faking::data::defaults::longitude_latitude::coordinates),
        "UUID V4" => FNVARI::String(data_faking::data::defaults::uuids::uuid_v4_wasm),

        //blockchain - bitcoin
        "Bitcoin Mainnet address" => FNVARI::String(data_faking::data::blockchain::bitcoin::mainnet_address),
        "Bitcoin Testnet address" => FNVARI::String(data_faking::data::blockchain::bitcoin::testnet_address),
        "Bitcoin Signet address" => FNVARI::String(data_faking::data::blockchain::bitcoin::signet_address),
        "Bitcoin Regtest address" => FNVARI::String(data_faking::data::blockchain::bitcoin::regtest_address),

        //blockchain - ethereum
        "Generate ethereum wallet address" => FNVARI::String(data_faking::data::blockchain::ethereum::generate_wallet_address),

        //religion - bible
        "Biblical Figures" => FNVARI::String(data_faking::locales::en::religion::bible::bible_characters),
        "Biblical Locations" => FNVARI::String(data_faking::locales::en::religion::bible::bible_locations),
        "Biblical Quotes" => FNVARI::String(data_faking::locales::en::religion::bible::bible_quotes),

        //media - elder scrolls
        "Elder Scrolls Characters" => FNVARI::String(data_faking::locales::en::media::games::elderscrolls::es_characters),
        "Elder Scrolls Games" => FNVARI::String(data_faking::locales::en::media::games::elderscrolls::es_games),
        "Elder Scrolls Locations" => FNVARI::String(data_faking::locales::en::media::games::elderscrolls::es_location),
        "Elder Scrolls Factions" => FNVARI::String(data_faking::locales::en::media::games::elderscrolls::es_factions),
        "Elder Scrolls Events" => FNVARI::String(data_faking::locales::en::media::games::elderscrolls::es_events),

        //media - friends
        "Friends Characters" => FNVARI::String(data_faking::locales::en::media::shows::friends::friends_characters),
        "Friends Quotes" => FNVARI::String(data_faking::locales::en::media::shows::friends::friends_quotes),
        "Friends Locations" => FNVARI::String(data_faking::locales::en::media::shows::friends::friends_locations),

        //media - games
        "Game Titles" => FNVARI::String(data_faking::locales::en::media::games::games::games_title),
        "Game Genres" => FNVARI::String(data_faking::locales::en::media::games::games::games_genre),
        "Game Platforms" => FNVARI::String(data_faking::locales::en::media::games::games::games_platform),

        //media - hp lovecraft
        "HP Lovecraft Dieties" => FNVARI::String(data_faking::locales::en::media::hp_lovecraft::lovecraft_artefact),
        "HP Lovecraft Books" => FNVARI::String(data_faking::locales::en::media::hp_lovecraft::lovecraft_book),
        "HP Lovecraft Locations" => FNVARI::String(data_faking::locales::en::media::hp_lovecraft::lovecraft_location),
        "HP Lovecraft Artefacts" => FNVARI::String(data_faking::locales::en::media::hp_lovecraft::lovecraft_artefact),

        //media - kpop
        "Kpop Groups (first generation)" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_group_first_generation),
        "Kpop Groups (second generation)" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_group_second_generation),
        "Kpop Groups (third generation)" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_group_third_generation),
        "Kpop Girl Groups" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_girl_groups),
        "Kpop Boy Bands" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_boy_bands),
        "Kpop Soloists" => FNVARI::String(data_faking::locales::en::media::korean::kpop::kpop_soloists),

        //media - lord of the rings
        "Lord of the Rings Characters" => FNVARI::String(data_faking::locales::en::media::shows::lord_of_the_rings::lord_of_the_rings_characters),
        "Lord of the Rings Quotes" => FNVARI::String(data_faking::locales::en::media::shows::lord_of_the_rings::lord_of_the_rings_quotes),
        "Lord of the Rings Locations" => FNVARI::String(data_faking::locales::en::media::shows::lord_of_the_rings::lord_of_the_rings_locations),

        //media - manga
        "Manga Titles" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::manga::manga_title),
        "Manga Genres" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::manga::manga_genre),
        "Manga Platforms" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::manga::manga_platform),

        //media - mario
        "Mario Characters" => FNVARI::String(data_faking::locales::en::media::games::mario::mario_character),
        "Mario Games" => FNVARI::String(data_faking::locales::en::media::games::mario::mario_game),
        "Mario Locations" => FNVARI::String(data_faking::locales::en::media::games::mario::mario_location),

        //media - minecraft
        "Minecraft Achievements" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_achievement),
        "Minecraft Biomes" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_biome),
        "Minecraft Blocks" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_block),
        "Minecraft Enchantments" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_enchantment),
        "Minecraft Game Modes" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_game_mode),
        "Minecraft Items" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_item),
        "Minecraft Mobs" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_mob),
        "Minecraft Status Effects" => FNVARI::String(data_faking::locales::en::media::games::minecraft::minecraft_status_effect),

        //media - movies
        "Movies" => FNVARI::String(data_faking::locales::en::media::shows::movies::movies),

        //media - one piece
        "One Piece Characters" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::one_piece::one_piece_characters),
        "One Piece Quotes" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::one_piece::one_piece_quotes),
        "One Piece Locations" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::one_piece::one_piece_locations),
        "One Piece Islands" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::one_piece::one_piece_islands),
        "One Piece Seas" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::one_piece::one_piece_seas),

        //media - pokemon
        "Pokemon Names" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::pokemon::pokemon_names),
        "Pokemon Moves" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::pokemon::pokemon_moves),
        "Pokemon Locations" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::pokemon::pokemon_locations),

        //media - seinfeld
        "Seinfeld Characters" => FNVARI::String(data_faking::locales::en::media::shows::seinfeld::seinfeld_characters),
        "Seinfeld Quotes" => FNVARI::String(data_faking::locales::en::media::shows::seinfeld::seinfeld_quotes),
        "Seinfeld Businesses" => FNVARI::String(data_faking::locales::en::media::shows::seinfeld::seinfeld_businesses),

        //media - silicon valley
        "Silicon Valley Characters" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_characters),
        "Silicon Valley Companies" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_companies),
        "Silicon Valley Quotes" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_quotes),
        "Silicon Valley Apps" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_apps),
        "Silicon Valley Inventions" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_inventions),
        "Silicon Valley Mottos" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_mottos),
        "Silicon Valley URLs" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_urls),
        "Silicon Valley Emails" => FNVARI::String(data_faking::locales::en::media::shows::silicon_valley::silicon_valley_emails),

        //media - simpsons
        "Simpsons Characters" => FNVARI::String(data_faking::locales::en::media::shows::simpsons::simpsons_character),
        "Simpsons Quotes" => FNVARI::String(data_faking::locales::en::media::shows::simpsons::simpsons_quote),
        "Simpsons Locations" => FNVARI::String(data_faking::locales::en::media::shows::simpsons::simpsons_location),
        "Simpsons Episode Titles" => FNVARI::String(data_faking::locales::en::media::shows::simpsons::simpsons_episode_title),

        //media - spongebob
        "Spongebob Characters" => FNVARI::String(data_faking::locales::en::media::shows::spongebob::spongebob_character),
        "Spongebob Quotes" => FNVARI::String(data_faking::locales::en::media::shows::spongebob::spongebob_quote),
        "Spongebob Locations" => FNVARI::String(data_faking::locales::en::media::shows::spongebob::spongebob_location),
        "Spongebob Episode Titles" => FNVARI::String(data_faking::locales::en::media::shows::spongebob::spongebob_episode_title),

        //media - starwars
        "Star Wars Characters" => FNVARI::String(data_faking::locales::en::media::shows::starwars::starwars_character),
        "Star Wars Squadrons" => FNVARI::String(data_faking::locales::en::media::shows::starwars::call_squadron),
        "Star Wars Call Numbers" => FNVARI::String(data_faking::locales::en::media::shows::starwars::call_number),
        "Star Wars Call Signs" => FNVARI::String(data_faking::locales::en::media::shows::starwars::call_sign),
        "Star Wars Droids" => FNVARI::String(data_faking::locales::en::media::shows::starwars::droid),
        "Star Wars Planets" => FNVARI::String(data_faking::locales::en::media::shows::starwars::starwars_planet),
        "Star Wars Species" => FNVARI::String(data_faking::locales::en::media::shows::starwars::species),
        "Star Wars Vehicles" => FNVARI::String(data_faking::locales::en::media::shows::starwars::vehicle),
        "Star Wars Wookie Words" => FNVARI::String(data_faking::locales::en::media::shows::starwars::wookie_word),

        //media - starwars (yoda)
        "Yoda Quotes" => FNVARI::String(data_faking::locales::en::media::shows::starwars_yoda::yoda),

        //media - studio ghibli
        "Studio Ghibli Characters" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::studio_ghibli::studio_ghibli_characters),
        "Studio Ghibli Quotes" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::studio_ghibli::studio_ghibli_quotes),
        "Studio Ghibli Movies" => FNVARI::String(data_faking::locales::en::media::anime_and_manga::studio_ghibli::studio_ghibli_movies),

        //media - the hobbit
        "The Hobbit Characters" => FNVARI::String(data_faking::locales::en::media::shows::the_hobbit::the_hobbit_characters),
        "The Hobbit Quotes" => FNVARI::String(data_faking::locales::en::media::shows::the_hobbit::the_hobbit_quotes),
        "The Hobbit Locations" => FNVARI::String(data_faking::locales::en::media::shows::the_hobbit::the_hobbit_locations),
        "The Hobbit Thorin's Company Members" => FNVARI::String(data_faking::locales::en::media::shows::the_hobbit::the_hobbit_thorins_company),

        //media - tolkien
        "Tolkien Characters" => FNVARI::String(data_faking::locales::en::media::shows::tolkein::tolkein_characters),
        "Tolkien Poems" => FNVARI::String(data_faking::locales::en::media::shows::tolkein::tolkein_poems),
        "Tolkien Locations" => FNVARI::String(data_faking::locales::en::media::shows::tolkein::tolkein_locations),
        "Tolkien Races" => FNVARI::String(data_faking::locales::en::media::shows::tolkein::tolkein_races),

        //misc = adjective
        "Positive Adjectives" => FNVARI::String(data_faking::locales::en::misc::adjective::positive_adjective),
        "Negative Adjectives" => FNVARI::String(data_faking::locales::en::misc::adjective::negative_adjective),

        // misc - Agent Bo
        // TODO: This function requires one arguments
        // "Bot Agents" => FNVARI::String(data_faking::data::it::agent_bot::bot_agent),

        // misc - Ancients
        "Gods" => FNVARI::String(data_faking::locales::en::mythos::ancients::god),
        "Primordials" => FNVARI::String(data_faking::locales::en::mythos::ancients::primordial),
        "Titans" => FNVARI::String(data_faking::locales::en::mythos::ancients::titan),
        "Heros" => FNVARI::String(data_faking::locales::en::mythos::ancients::hero),

        // misc - Animals
        "Animals" => FNVARI::String(data_faking::locales::en::animal::animals::animal),

        // misc - Appliances
        "Brands" => FNVARI::String(data_faking::locales::en::business::appliances::brand),
        "Equipments" => FNVARI::String(data_faking::locales::en::business::appliances::equipment),

        // misc - Artists
        "artists" => FNVARI::String(data_faking::locales::en::misc::artists::artist),

        // misc - barcodes
        "UPC-A Barcode" => FNVARI::String(data_faking::locales::en::misc::barcodes::upc_a),
        "UPC-A Barcode with Composite Symbology" => FNVARI::String(data_faking::locales::en::misc::barcodes::upc_a_with_composite_symbology),
        "UPC-E Barcode" => FNVARI::String(data_faking::locales::en::misc::barcodes::upc_e),
        "UPC-E Barcode with Composite Symbology" => FNVARI::String(data_faking::locales::en::misc::barcodes::upc_e_with_composite_symbology),
        "ISBN Barcode" => FNVARI::String(data_faking::locales::en::misc::barcodes::isbn),
        "ISMN Barcode" => FNVARI::String(data_faking::locales::en::misc::barcodes::ismn),
        "ISSN Barcode" => FNVARI::String(data_faking::locales::en::misc::barcodes::issn),

        // misc - blood
        "Blood Types" => FNVARI::String(data_faking::locales::en::person::blood::blood_type),
        "Blood: RH Factorys" => FNVARI::String(data_faking::locales::en::person::blood::rh_factory),
        "Blood: Groups" => FNVARI::String(data_faking::locales::en::person::blood::group),

        // misc - books
        "Book Titles" => FNVARI::String(data_faking::locales::en::misc::books::title),
        "Book Publishers" => FNVARI::String(data_faking::locales::en::misc::books::publisher),

        // misc - business
        "Business Categories" => FNVARI::String(data_faking::locales::en::business::business::category),

        // misc - chess
        "Chess Player" => FNVARI::String(data_faking::locales::en::misc::chess::chess_player),
        "Chess Tournaments" => FNVARI::String(data_faking::locales::en::misc::chess::chess_tournaments),
        "Chess Openings" => FNVARI::String(data_faking::locales::en::misc::chess::chess_opening),
        "Chess Titles" => FNVARI::String(data_faking::locales::en::misc::chess::chess_titles),
        "Chess Square Names" => FNVARI::String(data_faking::locales::en::misc::chess::chess_square_name),
        "Chess Piece Names" => FNVARI::String(data_faking::locales::en::misc::chess::chess_piece_name),

        // misc - codes
        "ASIN Codes" => FNVARI::String(data_faking::locales::en::misc::codes::asin),

        // misc - commerce
        "Departments" => FNVARI::String(data_faking::locales::en::business::commerce::department),
        "Product Adjectives" => FNVARI::String(data_faking::locales::en::business::commerce::product_adjective),
        "Materials" => FNVARI::String(data_faking::locales::en::business::commerce::material),
        "Products" => FNVARI::String(data_faking::locales::en::business::commerce::product),
        "Promotion Code Adjectives" => FNVARI::String(data_faking::locales::en::business::commerce::promotion_code_adjective),
        "Promotion Code Nouns" => FNVARI::String(data_faking::locales::en::business::commerce::promotion_code_noun),
        "Brand Commerces" => FNVARI::String(data_faking::locales::en::business::commerce::brand_commerce),
        "Brand Vendors" => FNVARI::String(data_faking::locales::en::business::commerce::vendor),
        "Full Product Names" => FNVARI::String(data_faking::locales::en::business::commerce::full_product_name),
        "Full Promotion Codes" => FNVARI::String(data_faking::locales::en::business::commerce::full_promotion_code),
        "Full Product Info" => FNVARI::String(data_faking::locales::en::business::commerce::full_product_info),

        // misc - compass
        "Cardinal Words" => FNVARI::String(data_faking::locales::en::misc::compass::cardinal_words),
        "Cardinal Abbreviations" => FNVARI::String(data_faking::locales::en::misc::compass::cardinal_abbreviation),
        "Cardinal Azimuths" => FNVARI::String(data_faking::locales::en::misc::compass::cardinal_azimuth),
        "Ordinal Words" => FNVARI::String(data_faking::locales::en::misc::compass::ordinal_words),
        "Ordinal Abbreviations" => FNVARI::String(data_faking::locales::en::misc::compass::ordinal_abbreviation),
        "Ordinal Azimuths" => FNVARI::String(data_faking::locales::en::misc::compass::ordinal_azimuth),
        "Half-wind Words" => FNVARI::String(data_faking::locales::en::misc::compass::half_wind_words),
        "Half-wind Abbreviations" => FNVARI::String(data_faking::locales::en::misc::compass::half_wind_abbreviation),
        "Half-wind Azimuths" => FNVARI::String(data_faking::locales::en::misc::compass::half_wind_azimuth),
        "Quarter-wind Words" => FNVARI::String(data_faking::locales::en::misc::compass::quarter_wind_words),
        "Quarter-wind Abbreviations" => FNVARI::String(data_faking::locales::en::misc::compass::quarter_wind_abbreviation),
        "Quarter-wind Azimuths" => FNVARI::String(data_faking::locales::en::misc::compass::quarter_wind_azimuth),

        // misc - Cryptocurrency
        "Cryptocurrency Names" => FNVARI::String(data_faking::data::blockchain::cryptocurrency::cryptocurrency_name),
        "Cryptocurrency Symbols" => FNVARI::String(data_faking::data::blockchain::cryptocurrency::cryptocurrency_symbol),

        // misc - Currencies
        "Currency Names" => FNVARI::String(data_faking::locales::en::misc::currencies::names),
        "Currency Symbols" => FNVARI::String(data_faking::locales::en::misc::currencies::symbols),
        "Currency Codes" => FNVARI::String(data_faking::locales::en::misc::currencies::codes),

        // misc - Dates
        // TODO: This function requires arguments
        // "Dates" => FNVARI::String(data_faking::data::defaults::date::random_date),
        // "Day of Week" => FNVARI::String(data_faking::data::defaults::date::random_date_dow),
        // "Future Date" => FNVARI::String(data_faking::data::defaults::date::random_date_future),
        // "Past Date" => FNVARI::String(data_faking::data::defaults::date::random_date_past),

        // misc - Demographic
        "Races" => FNVARI::String(data_faking::locales::en::person::demographic::race),
        "Sexes" => FNVARI::String(data_faking::locales::en::person::demographic::sex),
        "Denomyns" => FNVARI::String(data_faking::locales::en::person::demographic::demonym),
        "Educational Attainments" => FNVARI::String(data_faking::locales::en::person::demographic::educational_attainment),
        "Marital Statuses" => FNVARI::String(data_faking::locales::en::person::demographic::martial_status),

        // misc - Device
        "Model Names" => FNVARI::String(data_faking::data::it::device::model_name),
        "Manufacturer Names" => FNVARI::String(data_faking::data::it::device::manufacturer),
        "Serial Codes" => FNVARI::String(data_faking::data::it::device::serial),

        // misc - fashion
        "Fashion Terms" => FNVARI::String(data_faking::locales::en::misc::fashion::terms),

        // misc - food
        "Allergens" => FNVARI::String(data_faking::locales::en::misc::food::allergens),
        "Dishes" => FNVARI::String(data_faking::locales::en::misc::food::dishes),
        "Food Descriptions" => FNVARI::String(data_faking::locales::en::misc::food::descriptions),
        "Ingredients" => FNVARI::String(data_faking::locales::en::misc::food::ingredients),
        "Fruits" => FNVARI::String(data_faking::locales::en::misc::food::fruits),
        "Vegetables" => FNVARI::String(data_faking::locales::en::misc::food::vegetables),
        "Spices" => FNVARI::String(data_faking::locales::en::misc::food::spices),
        "Food Measurements" => FNVARI::String(data_faking::locales::en::misc::food::measurements),
        "Measurement Sizes" => FNVARI::String(data_faking::locales::en::misc::food::measurement_sizes),
        "Food Measurements (Metric)" => FNVARI::String(data_faking::locales::en::misc::food::metric_measurements),
        "Sushis" => FNVARI::String(data_faking::locales::en::misc::food::sushis),
        "Ethnic Categories (Food)" => FNVARI::String(data_faking::locales::en::misc::food::ethnic_categories),

        // misc - Greek Philosopher
        "Greek Philosopher Names" => FNVARI::String(data_faking::locales::en::mythos::greek_philosophers::greek_philosopher_names),
        "Greek Philosopher Quotes" => FNVARI::String(data_faking::locales::en::mythos::greek_philosophers::greek_philosopher_quotes),

        // misc - Industry Segment
        "Industries" => FNVARI::String(data_faking::locales::en::business::industry_segments::industry),
        "Super Sectors" => FNVARI::String(data_faking::locales::en::business::industry_segments::super_sector),
        "Sectors" => FNVARI::String(data_faking::locales::en::business::industry_segments::sector),
        "Subsectors" => FNVARI::String(data_faking::locales::en::business::industry_segments::sub_sector),

        // misc - IPV4
        "IPV4 Address" => FNVARI::String(data_faking::data::it::ipv4::ipv4_address),
        "IPV4 Address With CIDR" => FNVARI::String(data_faking::data::it::ipv4::ipv4_address_with_cidr),
        "Public IPV4 Address" => FNVARI::String(data_faking::data::it::ipv4::ipv4_address_public),
        "Private IPV4 Address" => FNVARI::String(data_faking::data::it::ipv4::ipv4_address_private),

        // misc - IPV6
        "IPV6 Address" => FNVARI::String(data_faking::data::it::ipv6::ipv6_address),
        "IPV6 Address With CIDR" => FNVARI::String(data_faking::data::it::ipv6::ipv6_address_with_cidr),

        // misc - jobs
        "Employment - Fields" => FNVARI::String(data_faking::locales::en::person::job::field),
        "Employment - Seniorities" => FNVARI::String(data_faking::locales::en::person::job::seniority),
        "Employment - Positions" => FNVARI::String(data_faking::locales::en::person::job::position),
        "Employment - Key Skills" => FNVARI::String(data_faking::locales::en::person::job::key_skill),
        "Employment - Types" => FNVARI::String(data_faking::locales::en::person::job::employment_type),
        "Employment - Education Levels" => FNVARI::String(data_faking::locales::en::person::job::education_level),

        // misc - Lorem Ipsu
        "Lorem Ipsum Word" => FNVARI::String(data_faking::locales::en::misc::lorem_ipsum::lorem_ipsum_word),
        "Lorem Ipsum Sentence" => FNVARI::String(data_faking::locales::en::misc::lorem_ipsum::lorem_ipsum_sentence),
        "Lorem Ipsum Paragraph" => FNVARI::String(data_faking::locales::en::misc::lorem_ipsum::lorem_ipsum_paragraph),

        // TODO: This function requires one arguments
        // "Lorem Ipsum Paragraphs" => FNVARI::String(data_faking::locales::en::misc::lorem_ipsum::lorem_ipsum_paragraphs),

        // misc - MAC Addres
        // TODO: This function requires one arguments
        // "MAC Addresses" => FNVARI::String(data_faking::data::it::mac_address::mac_address),

        // misc - Marketing Buzzwor
        "Marketing Buzzwords" => FNVARI::String(data_faking::locales::en::business::marketing::marketing_buzzword),

        // misc - Measurements
        "Height Measurements" => FNVARI::String(data_faking::locales::en::misc::measurements::height_measurement),
        "Length Measurements" => FNVARI::String(data_faking::locales::en::misc::measurements::length_measurement),
        "Volume Measurements" => FNVARI::String(data_faking::locales::en::misc::measurements::volume_measurement),
        "Weight Measurements" => FNVARI::String(data_faking::locales::en::misc::measurements::weight_measurement),
        "Height Measurements (Metric)" => FNVARI::String(data_faking::locales::en::misc::measurements::metric_height_measurement),
        "Length Measurements (Metric)" => FNVARI::String(data_faking::locales::en::misc::measurements::metric_length_measurement),
        "Volume Measurements (Metric)" => FNVARI::String(data_faking::locales::en::misc::measurements::metric_volume_measurement),
        "Weight Measurements (Metric)" => FNVARI::String(data_faking::locales::en::misc::measurements::metric_weight_measurement),

        // misc - Military
        "Army Ranks" => FNVARI::String(data_faking::locales::en::person::military::army_rank),
        "Marine Ranks" => FNVARI::String(data_faking::locales::en::person::military::marine_rank),
        "Navy Ranks" => FNVARI::String(data_faking::locales::en::person::military::navy_rank),
        "Coast Guard Ranks" => FNVARI::String(data_faking::locales::en::person::military::coast_guard_rank),
        "Air Force Ranks" => FNVARI::String(data_faking::locales::en::person::military::air_force_rank),
        "Space Force Ranks" => FNVARI::String(data_faking::locales::en::person::military::space_force_rank),
        "Department of Defense Paygrades" => FNVARI::String(data_faking::locales::en::person::military::dod_paygrade),

        // misc - Programming Language
        "Programming Language Names" => FNVARI::String(data_faking::data::it::programming_languages::name),
        "Programming Language Creators" => FNVARI::String(data_faking::data::it::programming_languages::creator),

        // misc - Quotes
        "Motivational Quotes" => FNVARI::String(data_faking::locales::en::misc::quotes::motivational_quote),
        "Philisophical Quotes" => FNVARI::String(data_faking::locales::en::misc::quotes::philisophical_quote),
        "Movie Quotes" => FNVARI::String(data_faking::locales::en::misc::quotes::movie_quote),

        // misc - Relationship
        "Familial Direct" => FNVARI::String(data_faking::locales::en::person::relationship::familial_direct),
        "Familial Extended" => FNVARI::String(data_faking::locales::en::person::relationship::familial_extended),
        "In-Laws" => FNVARI::String(data_faking::locales::en::person::relationship::in_law),
        "Spouses" => FNVARI::String(data_faking::locales::en::person::relationship::spouse),
        "Parents" => FNVARI::String(data_faking::locales::en::person::relationship::parent),
        "Siblings" => FNVARI::String(data_faking::locales::en::person::relationship::sibling),

        // misc - Restaurant
        "Restaurant Name Prefixes" => FNVARI::String(data_faking::locales::en::misc::restaurant::name_prefix),
        "Restaurant Name Suffixes" => FNVARI::String(data_faking::locales::en::misc::restaurant::name_suffix),
        "Restaurant Full Names" => FNVARI::String(data_faking::locales::en::misc::restaurant::restaurant_full_name),
        "Restaurant Types" => FNVARI::String(data_faking::locales::en::misc::restaurant::restaurant_type),
        "Restaurant Descriptions" => FNVARI::String(data_faking::locales::en::misc::restaurant::restaurant_description),
        "Restaurant Reviews" => FNVARI::String(data_faking::locales::en::misc::restaurant::restaurant_review),
        "Restaurants (All Details)" => FNVARI::String(data_faking::locales::en::misc::restaurant::generate_full_restaurant),

        // misc - Shakespeare
        "Shakespeare - Hamlet Quotes" => FNVARI::String(data_faking::locales::en::misc::shakespeare::shakespeare_hamlet_quotes),
        "Shakespeare - King Richard III Quotes" => FNVARI::String(data_faking::locales::en::misc::shakespeare::shakespeare_king_richard_iii_quotes),
        "Shakespeare - As You Like It Quotes" => FNVARI::String(data_faking::locales::en::misc::shakespeare::shakespeare_as_you_like_it_quotes),
        "Shakespeare - Romeo and Juliet Quotes" => FNVARI::String(data_faking::locales::en::misc::shakespeare::shakespeare_romeo_and_juliet_quotes),

        // misc - Space
        "Planets" => FNVARI::String(data_faking::locales::en::misc::space::planet),
        "Moons" => FNVARI::String(data_faking::locales::en::misc::space::moon),
        "Galaxies" => FNVARI::String(data_faking::locales::en::misc::space::galaxy),
        "Nebulas" => FNVARI::String(data_faking::locales::en::misc::space::nebula),
        "Star Clusters" => FNVARI::String(data_faking::locales::en::misc::space::star_cluster),
        "Constellations" => FNVARI::String(data_faking::locales::en::misc::space::constellation),
        "Stars" => FNVARI::String(data_faking::locales::en::misc::space::star),
        "Agencies" => FNVARI::String(data_faking::locales::en::misc::space::agency),
        "Agency Abbreviations" => FNVARI::String(data_faking::locales::en::misc::space::agency_abv),
        "NASA Space Crafts" => FNVARI::String(data_faking::locales::en::misc::space::nasa_space_craft),
        "Space Companies" => FNVARI::String(data_faking::locales::en::misc::space::company),
        "Distant Measurements (Space)" => FNVARI::String(data_faking::locales::en::misc::space::distance_measurement),
        "Meteorites" => FNVARI::String(data_faking::locales::en::misc::space::meteorite),
        "Launch Vehicles" => FNVARI::String(data_faking::locales::en::misc::space::launch_vehicle),

        // misc - Sports
        "Summer Olympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::summer_olympic),
        "Winter Olympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::winter_olympic),
        "Summer Paralympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::summer_paralympic),
        "Winter Paralympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::winter_paralympics),
        "Ancient Olympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::ancient_olympic),
        "Unusual Olympics Sports" => FNVARI::String(data_faking::locales::en::misc::sports::unusual),

        // misc - Stripe
        "Card Data (Stripe)" => FNVARI::CardData(data_faking::data::api::stripe::card::valid_card),
        "Token Data (Stripe)" => FNVARI::TokenData(data_faking::data::api::stripe::card::valid_token),
        "Invalid Card Data (Stripe)" => FNVARI::InvalidCardData(data_faking::data::api::stripe::card::invalid_token),

        // misc - Subscription
        "Subscription Plan" => FNVARI::String(data_faking::data::api::stripe::subscription::plan),
        "Subscription Status" => FNVARI::String(data_faking::data::api::stripe::subscription::status),
        "Subscription Method" => FNVARI::String(data_faking::data::api::stripe::subscription::payment_method),
        "Subscription Term" => FNVARI::String(data_faking::data::api::stripe::subscription::subscription_term),
        "Subscription Payment Term" => FNVARI::String(data_faking::data::api::stripe::subscription::payment_term),

        // misc - Tea
        "Black Tea" => FNVARI::String(data_faking::locales::en::misc::tea::black_tea),
        "Oolong Tea" => FNVARI::String(data_faking::locales::en::misc::tea::oolong_tea),
        "Green Tea" => FNVARI::String(data_faking::locales::en::misc::tea::green_tea),
        "White Tea" => FNVARI::String(data_faking::locales::en::misc::tea::white_tea),
        "Herbal Tea" => FNVARI::String(data_faking::locales::en::misc::tea::herbal_tea),
        "Tea Type" => FNVARI::String(data_faking::locales::en::misc::tea::tea_type),

        //datetime - date_naive
        "Date Naive" => FNVARI::String(data_faking::data::datetime::date_naive::date_naive),

        // TODO: This function requires one arguments
        // "Date Naive Before Today" => FNVARI::String(data_faking::data::datetime::date_naive::date_naive_before),
        // "Date Naive After Today" => FNVARI::String(data_faking::data::datetime::date_naive::date_naive_after),

        //datetime - datetime_naive
        "Datetime Naive" => FNVARI::String(data_faking::data::datetime::datetime_naive::datetime_naive),
        "Datetime Naive Milisecond" => FNVARI::String(data_faking::data::datetime::datetime_naive::datetime_naive_milli),
        "Datetime Naive Microsecond" => FNVARI::String(data_faking::data::datetime::datetime_naive::datetime_naive_micro),
        "Datetime Naive Nanosecond" => FNVARI::String(data_faking::data::datetime::datetime_naive::datetime_naive_nano),

        //datetime - month
        "Month" => FNVARI::String(data_faking::data::datetime::month::month),
        "Month Abbreviated" => FNVARI::String(data_faking::data::datetime::month::month_abbr),
        "Month Ordinal" => FNVARI::String(data_faking::data::datetime::month::month_ordinal),
        "Month Ordinal 0" => FNVARI::String(data_faking::data::datetime::month::month_ordinal0),

        //datetime - time_naive
        "Time Naive" => FNVARI::String(data_faking::data::datetime::time_naive::time_naive),
        "Time Naive Millisecond" => FNVARI::String(data_faking::data::datetime::time_naive::time_naive_milli),
        "Time Naive Micro" => FNVARI::String(data_faking::data::datetime::time_naive::time_naive_micro),
        "Time Naive Nano" => FNVARI::String(data_faking::data::datetime::time_naive::time_naive_nano),

        //datetime - unix
        "Unix Timestamp" => FNVARI::String(data_faking::data::datetime::unix::unix_ts),

        //datetime - weekday
        "Weekday" => FNVARI::String(data_faking::data::datetime::weekday::weekday),
        "Weekday Abbreviated" => FNVARI::String(data_faking::data::datetime::weekday::weekday_abbr),

        //datetime - year
        "Year" => FNVARI::String(data_faking::data::datetime::year::year),

        //datetime - sql
        "SQL Time" => FNVARI::String(data_faking::data::datetime::sql::sql_time),
        "SQL Server Time" => FNVARI::String(data_faking::data::datetime::sql::sql_server_time),
        "SQL Date" => FNVARI::String(data_faking::data::datetime::sql::sql_date),
        "SQL Datetime" => FNVARI::String(data_faking::data::datetime::sql::sql_datetime),
        "SQL Server DateTime" => FNVARI::String(data_faking::data::datetime::sql::sql_server_datetime),
        "SQL Server DateTime2" => FNVARI::String(data_faking::data::datetime::sql::sql_server_datetime2),

        // Canada
        "Canada - Street Suffix" => FNVARI::String(data_faking::data::countries::canada::addresses::street_suffix),
        "Canada - Street Name" => FNVARI::String(data_faking::data::countries::canada::addresses::street_name),
        // CANADA_CITY = "Canada - City",
        "Canada - Province Code" => FNVARI::String(data_faking::data::countries::canada::addresses::province),
        "Canada - Street Address" => FNVARI::String(data_faking::data::countries::canada::addresses::street_address),
        "Canada - Secondary Address" => FNVARI::String(data_faking::data::countries::canada::addresses::secondary_address),
        "Canada - Full Address" => FNVARI::String(data_faking::data::countries::canada::addresses::full_address),

        // USA
        "USA - City Prefix" => FNVARI::String(data_faking::data::countries::usa::addresses::city_prefix),
        "USA - City Suffix" => FNVARI::String(data_faking::data::countries::usa::addresses::city_suffix),
        "USA - Street Suffix" => FNVARI::String(data_faking::data::countries::usa::addresses::street_suffix),
        "USA - Street Name" => FNVARI::String(data_faking::data::countries::usa::addresses::street_name),
        "USA - State" => FNVARI::String(data_faking::data::countries::usa::addresses::state),
        "USA - City" => FNVARI::String(data_faking::data::countries::usa::addresses::city),
        // USA_ZIP_CODE = "USA - Zip Code",
        "USA - Street Address" => FNVARI::String(data_faking::data::countries::usa::addresses::street_address),
        "USA - Secondary Address" => FNVARI::String(data_faking::data::countries::usa::addresses::secondary_address),
        "USA - Full Address" => FNVARI::String(data_faking::data::countries::usa::addresses::full_address),

        _ => {
            eprintln!("\"{}\" - no generator found for this string. Try using the `faking list` command to see available generators.", arg);
            process::exit(0);
        }
    }
}
