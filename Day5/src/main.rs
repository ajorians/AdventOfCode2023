use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

type long = i128;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

struct MappingItem
{
    destination : long,
    source : long,
    length : long
}

struct Mappings
{
    SeedToSoilMap : Vec<MappingItem>,
    SoilToFertilizerMap : Vec<MappingItem>,
    FertilizerToWaterMap : Vec<MappingItem>,
    WaterToLightMap : Vec<MappingItem>,
    LightToTemperatureMap : Vec<MappingItem>,
    TemperatureToHumidityMap : Vec<MappingItem>,
    HumidityToLocationMap : Vec<MappingItem>
}

fn load_map( mapping_lines : &Vec<String>, category : &str ) -> Vec<MappingItem>
{
    let mut map : Vec<MappingItem> = Vec::new();

    let lines_of_category = mapping_lines.iter().skip_while(|line|
        {
            return line.starts_with(category) == false;
        })
        .take_while(|line|
            {
                return line.is_empty() != true;
            }).collect::<Vec<_>>();

    for line in lines_of_category
    {
        if line.ends_with(":")
        {
            continue;
        }

        let values: Vec<long> = line.trim().split(' ').flat_map(str::parse::<long>).collect::<Vec<_>>();

        let item : MappingItem = MappingItem
        {
            destination : values[0],
            source : values[1],
            length : values[2]
        };

        map.push( item );
    }

    return map;
}

fn load_mappings( mapping_lines : &Vec<String> ) -> Mappings
{
    let mapping : Mappings = Mappings
    {
        SeedToSoilMap : load_map( mapping_lines, "seed-to-soil map:"),
        SoilToFertilizerMap : load_map( mapping_lines, "soil-to-fertilizer map:"),
        FertilizerToWaterMap : load_map( mapping_lines, "fertilizer-to-water map:"),
        WaterToLightMap : load_map( mapping_lines, "water-to-light map:"),
        LightToTemperatureMap : load_map( mapping_lines, "light-to-temperature map:"),
        TemperatureToHumidityMap : load_map( mapping_lines, "temperature-to-humidity map:"),
        HumidityToLocationMap : load_map( mapping_lines, "humidity-to-location map:"),
    };

    return mapping;
}

fn get_seeds( seeds_line : &String )->Vec<long>
{
    let seeds = &seeds_line[6..];

    let seeds: Vec<long> = seeds.trim().split(' ').flat_map(str::parse::<long>).collect::<Vec<_>>();

    return seeds;
}

fn convert_item( mapping : &Vec<MappingItem>, value : long)->long
{
    for map_item in mapping
    {
        let offset = value - map_item.source;

        if value >= map_item.source && offset < map_item.length
        {
            let result = map_item.destination + offset;

            return result;
        }
    }

    return value;
}
fn seed_to_location( mappings : &Mappings, seed : long ) -> long
{
    let soil = convert_item( &mappings.SeedToSoilMap, seed );
    let fertilizer = convert_item( &mappings.SoilToFertilizerMap, soil );
    let water = convert_item( &mappings.FertilizerToWaterMap, fertilizer );
    let light = convert_item( &mappings.WaterToLightMap, water );
    let temperature = convert_item( &mappings.LightToTemperatureMap, light );
    let humidity = convert_item( &mappings.TemperatureToHumidityMap, temperature );
    let location = convert_item( &mappings.HumidityToLocationMap, humidity );

    return location;
}

fn main() {
    println!("Day 5");

    let lines = lines_from_file( "input.txt");

    let seeds : Vec<long> = get_seeds( &lines[0]);

    let mappings : Mappings = load_mappings( &lines );

    let mut lowest_location = long::max_value();
    for seed in seeds
    {
        let location = seed_to_location( &mappings, seed);

        println!("Seed {} is location {}", seed, location);

        if( location < lowest_location )
        {
            lowest_location = location;
        }
    }

    println!("Lowest is {}", lowest_location);


}
