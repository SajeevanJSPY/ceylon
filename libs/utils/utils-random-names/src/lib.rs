//! This small library just generates random names.
//!
//! Its purpose is to create identifiers that do not have to be
//! strictly unique like names for server nodes etc.
//!
//! The names are only usable for better reading where identifiers appear such as log files etc.
//!
//! This library is not meant to create fake data for testing.
extern crate rand;

use std::fmt::{Display, Formatter, Result};
use std::convert::Into;
use std::ops::Deref;
use rand::{Rng};
use rand::prelude::*;
use rand::rngs::OsRng;


/// A `RandomName` is just a container for a `String`.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomName {
    pub name: String,
}

impl RandomName {
    /// Creates a new `RandomName` that contains a random name.
    ///
    /// This function just calls `create_name` with a freshly generated random number generator.
    ///
    /// Internally this function tries to create an `OsRng` which can fail. Then the random name is not
    /// random anymore but "EDWIN ERROR".
    pub fn new() -> RandomName {
        let mut rng = OsRng;
        RandomName { name: create_name(&mut rng) }
    }
}

impl Display for RandomName {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

impl Deref for RandomName {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.name.as_ref()
    }
}

impl Into<String> for RandomName {
    fn into(self) -> String {
        self.name
    }
}

/// Creates a new `RandomName` given a random number generator.
pub fn create_name<T: Rng>(rng: &mut T) -> String {
    let name: &str = NAMES.choose(rng).copied().unwrap_or(NO_NAMES_NAME);
    let surname: &str = SURNAMES.choose(rng).copied().unwrap_or(NO_NAMES_SURNAME);
    format!("{} {}", name, surname)
}

const NO_NAMES_NAME: &'static str = "JOHN";
const NO_NAMES_SURNAME: &'static str = "DOE";

const NAMES: &'static [&'static str] = &["Lyric",
    "Bryce",
    "Cole",
    "Cristofer",
    "Kyan",
    "Enrique",
    "Reid",
    "Gage",
    "Baron",
    "Armando",
    "Davin",
    "Nash",
    "Jaiden",
    "Erick",
    "Jase",
    "Kamari",
    "Jamarion",
    "Adrian",
    "Arjun",
    "Donavan",
    "Julius",
    "Carson",
    "Abram",
    "Lewis",
    "Oscar",
    "Julien",
    "Ellis",
    "Enzo",
    "Fletcher",
    "Jamar",
    "Tyrone",
    "Aden",
    "Zaiden",
    "Chance",
    "Jerimiah",
    "Joey",
    "Bo",
    "Joel",
    "Humberto",
    "Zack",
    "Kael",
    "Jermaine",
    "Adam",
    "Clark",
    "Rylan",
    "Ross",
    "Kieran",
    "Brodie",
    "Sidney",
    "Sean",
    "Arthur",
    "Weston",
    "Trenton",
    "Conrad",
    "Dangelo",
    "Ian",
    "Isaias",
    "Soren",
    "Kyle",
    "Darien",
    "Larry",
    "Sage",
    "Matteo",
    "Jabari",
    "Kash",
    "Austin",
    "Curtis",
    "Jonathon",
    "Israel",
    "Octavio",
    "Everett",
    "Johnny",
    "Gustavo",
    "Brayden",
    "Hassan",
    "Dustin",
    "Moses",
    "Zayne",
    "Kadyn",
    "Albert",
    "David",
    "Zackary",
    "Gilbert",
    "Darrell",
    "Efrain",
    "Jon",
    "Carmelo",
    "Quinten",
    "Donald",
    "Keith",
    "Dominique",
    "Timothy",
    "Broderick",
    "Tyrell",
    "Zachery",
    "Wayne",
    "Marvin",
    "Cory",
    "Kaeden",
    "Gunnar",
    "Waylon",
    "Keyon",
    "Angel",
    "Abdiel",
    "Dylan",
    "Edward",
    "Roy",
    "Brock",
    "Alfredo",
    "Camren",
    "Rhys",
    "Jayce",
    "Elliott",
    "Felipe",
    "Randy",
    "Elijah",
    "Slade",
    "Wilson",
    "Randall",
    "Remington",
    "Lennon",
    "Richard",
    "Nasir",
    "Kellen",
    "Carl",
    "Kingston",
    "Kymani",
    "Ruben",
    "Gianni",
    "Ethan",
    "Misael",
    "Bradley",
    "Paxton",
    "Sebastian",
    "Boston",
    "Devan",
    "Case",
    "Conner",
    "Antonio",
    "Lukas",
    "Nick",
    "Alfonso",
    "Lane",
    "Andre",
    "Thomas",
    "Tanner",
    "Leland",
    "Kylan",
    "Barrett",
    "Jaden",
    "Logan",
    "Colton",
    "Axel",
    "Giovani",
    "Royce",
    "Caden",
    "Titus",
    "Vicente",
    "Leonel",
    "Jason",
    "Silas",
    "Damian",
    "Rocco",
    "Antony",
    "Kameron",
    "Chad",
    "Kane",
    "Beckett",
    "Cedric",
    "Shane",
    "Mathias",
    "Aaron",
    "Kade",
    "Luca",
    "Reece",
    "Seth",
    "Noe",
    "Ezequiel",
    "Grady",
    "Uriah",
    "Russell",
    "Emmanuel",
    "Rene",
    "Teagan",
    "Gavyn",
    "Jaxson",
    "Antwan",
    "Maurice",
    "Maxwell",
    "Brendon",
    "Justice",
    "Dax",
    "Kenny",
    "Damon",
    "Drake",
    "Tony",
    "Pedro",
    "Mathew",
    "Dennis",
    "Jorge",
    "Ashlyn",
    "Meadow",
    "Ava",
    "Ryan",
    "Makayla",
    "Livia",
    "Giada",
    "Vanessa",
    "Trinity",
    "Pamela",
    "Kianna",
    "Cristal",
    "Natasha",
    "Nadia",
    "Skylar",
    "Chloe",
    "Crystal",
    "Cherish",
    "Victoria",
    "Marisol",
    "Britney",
    "Micah",
    "Isabela",
    "Grace",
    "Ryleigh",
    "Jacquelyn",
    "Alejandra",
    "Kayley",
    "Alexia",
    "Cindy",
    "Sophia",
    "Renee",
    "Nia",
    "Neveah",
    "Sherlyn",
    "Josie",
    "Mareli",
    "Addisyn",
    "Amaris",
    "Madalynn",
    "Valery",
    "Olive",
    "Kinley",
    "Kyleigh",
    "Paloma",
    "Alice",
    "Adriana",
    "Emilee",
    "Jaden",
    "Isabella",
    "Ximena",
    "Chanel",
    "Reina",
    "Mattie",
    "Melina",
    "Jayla",
    "Audrey",
    "Maryjane",
    "Maritza",
    "Helen",
    "Patricia",
    "Sasha",
    "Paityn",
    "Alina",
    "Charlize",
    "Megan",
    "Marlee",
    "Jillian",
    "Evie",
    "Lily",
    "Camila",
    "Erika",
    "Paige",
    "Lina",
    "Lucille",
    "Alexus",
    "Natalya",
    "Jaylene",
    "Isabelle",
    "Elsa",
    "Jaida",
    "Lucia",
    "Fatima",
    "Elliana",
    "Lucy",
    "Kenley",
    "Nevaeh",
    "Jaiden",
    "Deja",
    "Justine",
    "Lola",
    "Angelina",
    "Tanya",
    "Princess",
    "Celeste",
    "Genesis",
    "Carley",
    "Emmalee",
    "Tess",
    "Hanna",
    "Alyson",
    "Karsyn",
    "Virginia",
    "Destiny",
    "Mireya",
    "Aniya",
    "Kimberly",
    "Ella",
    "Sage",
    "Cora",
    "Kayla",
    "Wendy",
    "Jolie",
    "Lila",
    "Samantha",
    "Riya",
    "Aryanna",
    "Krystal",
    "Rosemary",
    "Isabel",
    "Damaris",
    "Susan",
    "Kara",
    "Raelynn",
    "Cassidy",
    "Monica",
    "Alexandra",
    "Destiney",
    "Alison",
    "Johanna",
    "Anya",
    "Janessa",
    "Baylee",
    "Caitlyn",
    "Sierra",
    "Maribel",
    "Angeline",
    "Ashtyn",
    "Anahi",
    "Jaylynn",
    "Cailyn",
    "Giuliana",
    "Cristina",
    "Tania",
    "Karli",
    "Hailey",
    "Emerson",
    "Ellen",
    "Teagan",
    "Aaliyah",
    "Jamiya",
    "Giana",
    "Maliyah",
    "Emely",
    "Valerie",
    "Rachael",
    "Jadyn",
    "Kaia",
    "June",
    "Jenny",
    "Hadley",
    "Kennedi",
    "Selena",
    "Josephine",
    "Jazmyn",
    "Zoie",
    "Logan",
    "Lainey",
    "Mallory",
    "Maren",
    "Andrea",
    "Naima",
    "Clarissa",
    "Kaya",
    "Dalia",
    "Kailey",
    "Braelyn",
    "Faith",
    "Daniela",
    "Phoebe",
    "Diamond",
    "Deborah",
    "Genevieve",
    "Rayne",
    "Jade",
    "Reagan",
    "Brenna",
    "Michelle",
    "Jaelyn",
    "Jaqueline",
    "Regina",
    "Kylee",
    "Evelyn",
    "Valeria",
    "Kenna",
    "Amiyah",
    "Zaria",
    "Camilla",
    "Kate",
    "Laney",
    "Whitney",
    "Anika",
    "Erin",
    "Jayden",
    "Madeline",
    "Izabelle",
    "Melanie",
    "Kassidy",
    "Judith",
    "Harmony",
    "Fernanda",
    "Priscilla",
    "Cali",
    "Anabelle",
    "Nathaly",
    "Lana",
    "Haven",
    "Aliza",
    "Rachel",
    "Nayeli",
    "Annabella",
    "Yaritza",
    "Sonia",
    "Ariella",
    "Lisa",
    "Emmy",
    "Kaylah",
    "Kenya",
    "Miranda",
    "Chelsea",
    "Taryn",
    "Janelle",
    "Angelique",
    "Leilani",
    "Kadence",
    "Shyanne",
    "Marie",
    "Cameron",
    "Leia",
    "Jordyn",
    "Aylin",
    "Aimee",
    "Kiley",
    "Kamari",
    "Aniyah",
    "Kaylin",
    "Sanai",
    "Lilian",
    "Carlie",
    "Jayleen",
    "Hannah",
    "Dulce",
    "Jessie",
    "Natalie",
    "Giovanna",
    "Aryana",
    "Nylah",
    "Karley",
    "Alani",
    "Laurel",
    "Iliana",
    "Taniya",
    "Mckenna",
    "Rhianna",
    "Kyra",
    "Cynthia",
    "Anabel",
    "Salma",
    "Melody",
    "Armani",
    "Lena",
    "Hazel",
    "Melissa",
    "Layla",
    "Elisa",
    "Eleanor",
    "Tamia",
    "Linda",
    "Penelope",
    "Phoenix",
    "Jayda",
    "Melany",
    "Aracely",
    "Zoe",
    "Kiera",
    "Nataly",
    "Marianna",
    "Kamora",
    "Kierra",
    "Jocelynn",
    "Joselyn",
    "Shyla",
    "Desiree",
    "Liliana",
    "Tori",
    "Laura",
    "Guadalupe",
    "Iris",
    "Cheyenne",
    "Brisa",
    "Jakayla",
    "Summer",
    "Kaylen",
    "Irene",
    "Leyla",
    "Elaine",
    "Alena",
    "Liberty",
    "McKayla",
    "Mya",
    "Gia",
    "Jaslyn",
    "Gracie",
    "Haylie",
    "Brylee",
    "Amanda",
    "Eileen",
    "Kailee",
    "Janiya",
    "Anaya",
    "Keyla",
    "Amber",
    "Kaitlin",
    "Ashlee",
    "Shannon",
    "Alana",
    "Lea",
    "Perla",
    "Madelynn",
    "Kennedy",
    "Adison",
    "Kenzie",
    "Rylie",
    "Desirae",
    "Jimena",
    "Diya",
    "Shea",
    "Shania",
    "Iyana",
    "Aleena",
    "Emery",
    "Ayana",
    "Kaiya",
    "Justice",
    "Sienna",
    "Simone",
    "Naomi",
    "Saige",
    "Emelia",
    "Emilie",
    "Danika",
    "Juliana",
    "Nyasia",
    "Kiersten",
    "Janiah",
    "Nola",
    "Adalynn",
    "Reyna",
    "Ada",
    "Kaley",
    "Gina",
    "Aliya",
    "Cheyanne",
    "Felicity",
    "Camryn",
    "Madisyn",
    "Addison",
    "Cloe",
    "Malia",
    "Abigail",
    "Lia",
    "Macy",
    "Abril",
    "Kira",
    "Lindsey",
    "Sophie",
    "Esperanza",
    "Danna",
    "Zariah",
    "Laila",
    "Molly",
    "Jasmine",
    "Mikaela",
    "Audrina",
    "Alisa",
    "Gabriella",
    "Sofia",
    "Arielle",
    "Madilynn",
    "Angelica",
    "Carolina",
    "Scarlet",
    "Mackenzie",
    "Monique",
    "Essence",
    "Natalia",
    "Ruth",
    "Katrina",
    "Shelby",
    "Cara",
    "Freddy",
    "Gary",
    "Lakeisha",
    "Ione",
    "Margot",
    "Kary",
    "Dirk",
    "Rosella",
    "Erinn",
    "Mario",
    "Dalila",
    "Avis",
    "Rupert",
    "Dione",
    "Giovanni",
    "Ligia",
    "Alysia",
    "Coralie",
    "Helen",
    "Leigha",
    "Kathe",
    "Katrice",
    "Hermila",
    "Omar",
    "Ivory",
    "Deane",
    "Odilia",
    "Jaymie",
    "Brittaney",
    "Ofelia",
    "Sharonda",
    "Jayson",
    "Truman",
    "Lewis",
    "Georgiana",
    "Elanor",
    "Erma",
    "Riva",
    "Laureen",
    "Ouida",
    "Katina",
    "Mechelle",
    "Lyndon",
    "Gertie",
    "Jon",
    "Tisa",
    "Hayden",
    "Ty",
    "Jacklyn",
    "Mickie"];


const SURNAMES: &'static [&'static str] = &["Soto",
    "Maddox",
    "Weber",
    "Ward",
    "Brandt",
    "Shelton",
    "Wilkerson",
    "Schmitt",
    "Riley",
    "Leon",
    "Benson",
    "Everett",
    "Aguilar",
    "Brown",
    "Dalton",
    "Gonzalez",
    "Zavala",
    "Williamson",
    "Dodson",
    "Carlson",
    "Castro",
    "Christian",
    "Huynh",
    "Sanford",
    "Copeland",
    "Sharp",
    "Freeman",
    "Carpenter",
    "Herring",
    "Dillon",
    "Rowland",
    "Stanley",
    "Shields",
    "Newman",
    "Gross",
    "Harding",
    "Duke",
    "Higgins",
    "McMillan",
    "Floyd",
    "Pena",
    "Yates",
    "Butler",
    "Beard",
    "Burgess",
    "Fuller",
    "Lawrence",
    "Buchanan",
    "David",
    "Solis",
    "Gilmore",
    "Holden",
    "Petersen",
    "Schroeder",
    "Duarte",
    "Blevins",
    "Lynch",
    "Washington",
    "Sandoval",
    "Hunt",
    "Hebert",
    "Macdonald",
    "Mack",
    "Sheppard",
    "McKenzie",
    "Norris",
    "Dennis",
    "Tate",
    "Lyons",
    "Morgan",
    "Barajas",
    "Galvan",
    "Patel",
    "Santana",
    "Meza",
    "Martinez",
    "Banks",
    "Hughes",
    "Harvey",
    "Grant",
    "Clayton",
    "Coffey",
    "Pittman",
    "Hutchinson",
    "Collier",
    "Stevenson",
    "Melendez",
    "Murillo",
    "Rodgers",
    "Mercer",
    "Armstrong",
    "Morales",
    "Douglas",
    "Douven",
    "Blanchard",
    "Wallace",
    "Mckinney",
    "Bird",
    "Bradford",
    "Cervantes",
    "Church",
    "Gordon",
    "George",
    "Cochran",
    "Ayala",
    "Schaefer",
    "Frey",
    "French",
    "Obrien",
    "James",
    "Terry",
    "Leblanc",
    "Lewis",
    "Walton",
    "Walters",
    "Lloyd",
    "Barker",
    "Singh",
    "Montes",
    "Franklin",
    "Pennington",
    "Dunn",
    "Russo",
    "Austin",
    "Carter",
    "Reese",
    "Mejia",
    "Hurley",
    "Krueger",
    "Foley",
    "Jacobs",
    "Fox",
    "Miranda",
    "Woods",
    "Glenn",
    "Rich",
    "Frank",
    "Nash",
    "Lawson",
    "Keller",
    "Novak",
    "Holmes",
    "Bowen",
    "Casey",
    "Burton",
    "Guzman",
    "Vargas",
    "Riggs",
    "Atkins",
    "Lee",
    "Becker",
    "Potter",
    "Gallegos",
    "Hahn",
    "Nielsen",
    "Lane",
    "Archer",
    "Carr",
    "Mason",
    "Watson",
    "Hardin",
    "Fitzpatrick",
    "Snyder",
    "Webster",
    "Rosario",
    "Tanner",
    "Deleon",
    "Cross",
    "Solomon",
    "Beasley",
    "Ibarra",
    "Arellano",
    "Carney",
    "Garner",
    "Sexton",
    "Garrett",
    "Walker",
    "Gillespie",
    "Hall",
    "Maldonado",
    "Winters",
    "Barnes",
    "Robles",
    "Saunders",
    "Strong",
    "Cowan",
    "Harrell",
    "McGuire",
    "Hernandez",
    "Potts",
    "Reed",
    "Mooney",
    "Carey",
    "Gould",
    "Patterson",
    "Dawson",
    "Horton",
    "Farley",
    "Callahan",
    "Jensen",
    "English",
    "Abraham",
    "Allan",
    "Alsop",
    "Anderson",
    "Arnold",
    "Avery",
    "Bailey",
    "Baker",
    "Ball",
    "Bell",
    "Berry",
    "Black",
    "Blake",
    "Bond",
    "Bower",
    "Brown",
    "Buckland",
    "Burgess",
    "Butler",
    "Cameron",
    "Campbell",
    "Carr",
    "Chapman",
    "Churchill",
    "Clark",
    "Clarkson",
    "Coleman",
    "Cornish",
    "Davidson",
    "Davies",
    "Dickens",
    "Dowd",
    "Duncan",
    "Dyer",
    "Edmunds",
    "Ellison",
    "Ferguson",
    "Fisher",
    "Forsyth",
    "Fraser",
    "Gibson",
    "Gill",
    "Glover",
    "Graham",
    "Grant",
    "Gray",
    "Greene",
    "Hamilton",
    "Hardacre",
    "Harris",
    "Hart",
    "Hemmings",
    "Henderson",
    "Hill",
    "Hodges",
    "Howard",
    "Hudson",
    "Hughes",
    "Hunter",
    "Ince",
    "Jackson",
    "James",
    "Johnston",
    "Jones",
    "Kelly",
    "Kerr",
    "King",
    "Knox",
    "Lambert",
    "Langdon",
    "Lawrence",
    "Lee",
    "Lewis",
    "Lyman",
    "MacDonald",
    "Mackay",
    "Mackenzie",
    "MacLeod",
    "Manning",
    "Marshall",
    "Martin",
    "Mathis",
    "May",
    "McDonald",
    "McLean",
    "McGrath",
    "Metcalfe",
    "Miller",
    "Mills",
    "Mitchell",
    "Morgan",
    "Morrison",
    "Murray",
    "Nash",
    "Newman",
    "Nolan",
    "North",
    "Ogden",
    "Oliver",
    "Paige",
    "Parr",
    "Parsons",
    "Paterson",
    "Payne",
    "Peake",
    "Peters",
    "Piper",
    "Poole",
    "Powell",
    "Pullman",
    "Quinn",
    "Rampling",
    "Randall",
    "Rees",
    "Reid",
    "Roberts",
    "Robertson",
    "Ross",
    "Russell",
    "Rutherford",
    "Sanderson",
    "Scott",
    "Sharp",
    "Short",
    "Simpson",
    "Skinner",
    "Slater",
    "Smith",
    "Springer",
    "Stewart",
    "Sutherland",
    "Taylor",
    "Terry",
    "Thomson",
    "Tucker",
    "Turner",
    "Underwood",
    "Vance",
    "Vaughan",
    "Walker",
    "Wallace",
    "Walsh",
    "Watson",
    "Welch",
    "White",
    "Wilkins",
    "Wilson",
    "Wright",
    "Young"];