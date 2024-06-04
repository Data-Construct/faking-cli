pub fn print_categories() {
    println!(
        r#"Categories
api
blockchain
coding
countries
dates
finance
games
geo
media
misc
networking
people
types
religion"#
    );
}

pub fn print_category(category: &str) {
    match category {
        "api" => category_api(),
        "blockchain" => category_blockchain(),
        "coding" => category_coding(),
        "countries" => category_countries(),
        "dates" => category_dates(),
        "finance" => category_finance(),
        "games" => category_games(),
        "geo" => category_geo(),
        "media" => category_media(),
        "misc" => category_misc(),
        "networking" => category_networking(),
        "people" => category_people(),
        "types" => native_types(),
        "religion" => bible(),

        _ => {
            println!("Category not found.");
        }
    }
}

pub fn print_all() {
    native_types();

    category_people();

    category_coding();

    category_blockchain();

    category_networking();

    category_misc();

    bible();

    category_games();

    category_media();

    category_api();

    category_finance();

    category_dates();

    category_geo();

    category_countries();
}

pub fn category_coding() {
    lorem_ipsum();
    crypto_hashes();
}

pub fn category_dates() {
    datetime();
    sql_datetime();
}

pub fn category_finance() {
    currencies();
}

pub fn category_geo() {
    cordinates();
    compass();
}

pub fn category_api() {
    stripe();
}

pub fn category_networking() {
    ipv4();
    ipv6();
}

pub fn category_blockchain() {
    cryptocurrency();
    bitcoin();
    ethereum();
}

pub fn category_people() {
    names();
    phone_numbers();
    emails();
    relationships();
    demographic();
    blood();
}

pub fn category_countries() {
    usa_locations();
    canada_locations();
}

pub fn category_games() {
    minecraft();
    mario();
    games();
}

pub fn category_media() {
    tolkein();
    hobbit();
    studio_ghibli();
    starwars();
    spongebob();
    simpsons();
    silicon_valley();
    seinfeld();
    pokenmon();
    one_piece();
    movies();
    manga();
    lord_of_the_rings();
    kpop();
    lovecraft();
    friends();
    elder_scrolls();
}

pub fn category_misc() {
    tea();
    sports();
    space();
    shakespeare();
    restaurant();
    quotes();
    programming_languages();
    military();
    measurements();
    marketing();
    jobs();
    industry_segments();
    greek_philosophers();
    food();
    fashion();
    device();
    commerce();
    codes();
    chess();
    business();
    books();
    barcodes();
    artists();
    appliances();
    animals();
    ancients();
    adjective();
}

////

pub fn native_types() {
    println!(
        r#"---Native Types---
Bool

ISize
I8
I16
I32
I64

U8
U16
U32
U64
USize

F32
F64
"#
    );
}

pub fn names() {
    println!(
        r#"---Names---
Random Username

Neutral prefix

Male prefix standard
Male prefix
Male first name

Female prefix standard
Female prefix
Female first name

Neutral first name
Last name
"#
    );
}

pub fn phone_numbers() {
    println!(
        r#"---Phone Numbers---
Phone number with country code
Phone number
"#
    );
}

pub fn emails() {
    println!(
        r#"---Emails---
Standard generic email
Standard public email
Standard public email alias
Standard business email
Standard government email
"# // TODO: This function requires two arguments
           // Standard account email
    );
}

pub fn crypto_hashes() {
    println!(
        r#"---Crypto Hashes---
md5
sha1
sha256
sha512
"#
    );
}

pub fn cordinates() {
    println!(
        r#"---Cordinates---
Longitude
Latitude
Coordinates
"#
    );
}

pub fn bitcoin() {
    println!(
        r#"---Blockchain Bitcoin---
Bitcoin Mainnet address
Bitcoin Testnet address
Bitcoin Signet address
Bitcoin Regtest address
"#
    );
}

pub fn ethereum() {
    println!(
        r#"---Blockchain Ethereum---
Generate ethereum wallet address
"#
    );
}

pub fn bible() {
    println!(
        r#"---Religion Bible---
Biblical Figures
Biblical Locations
Biblical Quotes
"#
    );
}

pub fn elder_scrolls() {
    println!(
        r#"---Media Elder scrolls---
Elder Scrolls Characters
Elder Scrolls Games
Elder Scrolls Locations
Elder Scrolls Factions
Elder Scrolls Events
"#
    );
}

pub fn friends() {
    println!(
        r#"---Media Friends---
Friends Characters
Friends Quotes
Friends Locations
"#
    );
}

pub fn games() {
    println!(
        r#"---Games---
Game Titles
Game Genres
Game Platforms
"#
    );
}

pub fn lovecraft() {
    println!(
        r#"---Media HP lovecraft---
HP Lovecraft Dieties
HP Lovecraft Books
HP Lovecraft Locations
HP Lovecraft Artefacts
"#
    );
}

pub fn kpop() {
    println!(
        r#"---Media kpop---
Kpop Groups (first generation)
Kpop Groups (second generation)
Kpop Groups (third generation)
Kpop Girl Groups
Kpop Boy Bands
Kpop Soloists
"#
    );
}

pub fn lord_of_the_rings() {
    println!(
        r#"---Media Lord of the rings---
Lord of the Rings Characters
Lord of the Rings Quotes
Lord of the Rings Locations
"#
    );
}

pub fn manga() {
    println!(
        r#"---Media Manga---
Manga Titles
Manga Genres
Manga Platforms
"#
    );
}

pub fn mario() {
    println!(
        r#"---Games Mario---
Mario Characters
Mario Games
Mario Locations
"#
    );
}

pub fn minecraft() {
    println!(
        r#"---Games Minecraft---
Minecraft Achievements
Minecraft Biomes
Minecraft Blocks
Minecraft Enchantments
Minecraft Game Modes
Minecraft Items
Minecraft Mobs
Minecraft Status Effects
"#
    );
}

pub fn movies() {
    println!(
        r#"---Media Movies---
Movies
"#
    );
}

pub fn one_piece() {
    println!(
        r#"---Media One piece---
One Piece Characters
One Piece Quotes
One Piece Locations
One Piece Islands
One Piece Seas
"#
    );
}

pub fn pokenmon() {
    println!(
        r#"---Media Pokemon---
Pokemon Names
Pokemon Moves
Pokemon Locations
"#
    );
}

pub fn seinfeld() {
    println!(
        r#"---Media Seinfeld---
Seinfeld Characters
Seinfeld Quotes
Seinfeld Businesses
"#
    );
}

pub fn silicon_valley() {
    println!(
        r#"---Media Silicon Valley---
Silicon Valley Characters
Silicon Valley Companies
Silicon Valley Quotes
Silicon Valley Apps
Silicon Valley Inventions
Silicon Valley Mottos
Silicon Valley URLs
Silicon Valley Emails
"#
    );
}

pub fn simpsons() {
    println!(
        r#"---Media Simpsons---
Simpsons Characters
Simpsons Quotes
Simpsons Locations
Simpsons Episode Titles
"#
    );
}

pub fn spongebob() {
    println!(
        r#"---Media Spongebob---
Spongebob Characters
Spongebob Quotes
Spongebob Locations
Spongebob Episode Titles
"#
    );
}

pub fn starwars() {
    println!(
        r#"---Media Starwars---
Star Wars Characters
Star Wars Squadrons
Star Wars Call Numbers
Star Wars Call Signs
Star Wars Droids
Star Wars Planets
Star Wars Species
Star Wars Vehicles
Star Wars Wookie Words
Yoda Quotes
"#
    );
}

pub fn studio_ghibli() {
    println!(
        r#"---Media Studio Ghibli---
Studio Ghibli Characters
Studio Ghibli Quotes
Studio Ghibli Movies
"#
    );
}

pub fn hobbit() {
    println!(
        r#"---Media The hobbit---
The Hobbit Characters
The Hobbit Quotes
The Hobbit Locations
The Hobbit Thorin's Company Members
"#
    );
}

pub fn tolkein() {
    println!(
        r#"---Media Tolkien---
Tolkien Characters
Tolkien Poems
Tolkien Locations
Tolkien Races
"#
    );
}

pub fn adjective() {
    println!(
        r#"---Misc Adjective (English)---
Positive Adjectives
Negative Adjectives
"#
    );
}

pub fn ancients() {
    println!(
        r#"---Misc Ancients---
Gods
Primordials
Titans
Heros
"#
    );
}

pub fn animals() {
    println!(
        r#"---Misc Animals---
Animals
"#
    );
}

pub fn appliances() {
    println!(
        r#"---Misc Appliances---
Brands
Equipments
"#
    );
}

pub fn artists() {
    println!(
        r#"---Misc Artists---
artists
"#
    );
}

pub fn barcodes() {
    println!(
        r#"---Misc Barcodes---
UPC-A Barcode
UPC-A Barcode with Composite Symbology
UPC-E Barcode
UPC-E Barcode with Composite Symbology
ISBN Barcode
ISMN Barcode
ISSN Barcode
"#
    );
}

pub fn blood() {
    println!(
        r#"---Blood---
Blood Types
Blood: RH Factorys
Blood: Groups
"#
    );
}

pub fn books() {
    println!(
        r#"---Misc Books---
Book Titles
Book Publishers
"#
    );
}

pub fn business() {
    println!(
        r#"---Misc Business---
Business Categories
"#
    );
}

pub fn chess() {
    println!(
        r#"---Misc Chess---
Chess Player
Chess Tournaments
Chess Openings
Chess Titles
Chess Square Names
Chess Piece Names
"#
    );
}

pub fn codes() {
    println!(
        r#"---Misc Codes---
ASIN Codes
"#
    );
}

pub fn commerce() {
    println!(
        r#"---Misc Commerce---
Departments
Product Adjectives
Materials
Products
Promotion Code Adjectives
Promotion Code Nouns
Brand Commerces
Brand Vendors
Full Product Names
Full Promotion Codes
Full Product Info
"#
    );
}

pub fn compass() {
    println!(
        r#"---Compass---
Cardinal Words
Cardinal Abbreviations
Cardinal Azimuths
Ordinal Words
Ordinal Abbreviations
Ordinal Azimuths
Half-wind Words
Half-wind Abbreviations
Half-wind Azimuths
Quarter-wind Words
Quarter-wind Abbreviations
Quarter-wind Azimuths
"#
    );
}

pub fn cryptocurrency() {
    println!(
        r#"---Blockchain Cryptocurrency---
Cryptocurrency Names
Cryptocurrency Symbols
"#
    );
}

pub fn currencies() {
    println!(
        r#"---Misc Currencies---
Currency Names
Currency Symbols
Currency Codes
"#
    );
}

pub fn demographic() {
    println!(
        r#"---Demographic---
Races
Sexes
Denomyns
Educational Attainments
Marital Statuses
"#
    );
}

pub fn device() {
    println!(
        r#"---Misc Device---
Model Names
Manufacturer Names
Serial Codes
"#
    );
}

pub fn fashion() {
    println!(
        r#"---Misc Fashion---
Fashion Terms
"#
    );
}

pub fn food() {
    println!(
        r#"---Misc Food---
Allergens
Dishes
Food Descriptions
Ingredients
Fruits
Vegetables
Spices
Food Measurements
Measurement Sizes
Food Measurements (Metric)
Sushis
Ethnic Categories (Food)
"#
    );
}

pub fn greek_philosophers() {
    println!(
        r#"---Misc Greek Philosopher---
Greek Philosopher Names
Greek Philosopher Quotes
"#
    );
}

pub fn industry_segments() {
    println!(
        r#"---Misc Industry Segment---
Industries
Super Sectors
Sectors
Subsectors
"#
    );
}

pub fn ipv4() {
    println!(
        r#"---Networking IPV4---
IPV4 Address
IPV4 Address With CIDR
Public IPV4 Address
Private IPV4 Address
"#
    );
}

pub fn ipv6() {
    println!(
        r#"---Networking IPV6---
IPV6 Address
IPV6 Address With CIDR
"#
    );
}

pub fn jobs() {
    println!(
        r#"---Misc Jobs---
Employment - Fields
Employment - Seniorities
Employment - Positions
Employment - Key Skills
Employment - Types
Employment - Education Levels
"#
    );
}

pub fn lorem_ipsum() {
    println!(
        r#"---Lorem Ipsum---
Lorem Ipsum Word
Lorem Ipsum Sentence
Lorem Ipsum Paragraph
"#
    );
}

pub fn marketing() {
    println!(
        r#"---Misc Marketing---
Marketing Buzzwords
"#
    );
}

pub fn measurements() {
    println!(
        r#"---Misc Measurements---
Height Measurements
Length Measurements
Volume Measurements
Weight Measurements
Height Measurements (Metric)
Length Measurements (Metric)
Volume Measurements (Metric)
Weight Measurements (Metric)
"#
    );
}

pub fn military() {
    println!(
        r#"---Misc Military---
Army Ranks
Marine Ranks
Navy Ranks
Coast Guard Ranks
Air Force Ranks
Space Force Ranks
Department of Defense Paygrades
"#
    );
}

pub fn programming_languages() {
    println!(
        r#"---Misc Programming Language---
Programming Language Names
Programming Language Creators
"#
    );
}

pub fn quotes() {
    println!(
        r#"---Misc Quotes---
Motivational Quotes
Philisophical Quotes
Movie Quotes
"#
    );
}

pub fn relationships() {
    println!(
        r#"---Misc Relationship---
Familial Direct
Familial Extended
In-Laws
Spouses
Parents
Siblings
"#
    );
}

pub fn restaurant() {
    println!(
        r#"---Misc Restaurant---
Restaurant Name Prefixes
Restaurant Name Suffixes
Restaurant Full Names
Restaurant Types
Restaurant Descriptions
Restaurant Reviews
Restaurants (All Details)
"#
    );
}

pub fn shakespeare() {
    println!(
        r#"---Misc Shakespeare---
Shakespeare - Hamlet Quotes
Shakespeare - King Richard III Quotes
Shakespeare - As You Like It Quotes
Shakespeare - Romeo and Juliet Quotes
"#
    );
}

pub fn space() {
    println!(
        r#"---Misc Space---
Planets
Moons
Galaxies
Nebulas
Star Clusters
Constellations
Stars
Agencies
Agency Abbreviations
NASA Space Crafts
Space Companies
Distant Measurements (Space)
Meteorites
Launch Vehicles
"#
    );
}

pub fn sports() {
    println!(
        r#"---Misc Sports---
Summer Olympics Sports
Winter Olympics Sports
Summer Paralympics Sports
Winter Paralympics Sports
Ancient Olympics Sports
Unusual Olympics Sports
"#
    );
}

pub fn stripe() {
    println!(
        r#"---API Stripe---
Card Data (Stripe)
Token Data (Stripe)
Invalid Card Data (Stripe)

Subscription Plan
Subscription Status
Subscription Method
Subscription Term
Subscription Payment Term
"#
    );
}

pub fn tea() {
    println!(
        r#"---Misc Tea---
Black Tea
Oolong Tea
Green Tea
White Tea
Herbal Tea
Tea Type
"#
    );
}

pub fn datetime() {
    println!(
        r#"---Datetime---
Date Naive

Datetime Naive
Datetime Naive Milisecond
Datetime Naive Microsecond
Datetime Naive Nanosecond

Month
Month Abbreviated
Month Ordinal
Month Ordinal 0

Time Naive
Time Naive Millisecond
Time Naive Micro
Time Naive Nano

Unix Timestamp

Weekday
Weekday Abbreviated

Year
"#
    );
}

pub fn sql_datetime() {
    println!(
        r#"---SQL Datetime---
SQL Time
SQL Server Time
SQL Date
SQL Datetime
SQL Server DateTime
SQL Server DateTime2
"#
    );
}

pub fn canada_locations() {
    println!(
        r#"---Canada Locations---
Canada - Street Suffix
Canada - Street Name

Canada - Province Code
Canada - Street Address
Canada - Secondary Address
Canada - Full Address
"#
    );
}

pub fn usa_locations() {
    println!(
        r#"---USA Locations---
USA - City Prefix
USA - City Suffix
USA - Street Suffix
USA - Street Name
USA - State
USA - City

USA - Street Address
USA - Secondary Address
USA - Full Address
"#
    );
}
