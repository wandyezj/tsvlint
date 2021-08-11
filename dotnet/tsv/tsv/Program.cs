using System;
using System.IO;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace tsv
{
    public class Metadata
    {
        public string name { get; set; }
    }

    class Program
    {

        static void Main(string[] args)
        {
            Console.WriteLine("Hello World!");
            var datapath = "./data/";


            // Read JSON specification
            var pathMetadata = datapath + "metadata-simple.json";
            var data = File.ReadAllText(pathMetadata, System.Text.Encoding.UTF8);

            var metadata = JsonSerializer.Deserialize<Metadata>(data);
            Console.WriteLine(metadata.name);


            // Read Data file
            var path = "./data/data.tsv";
            var lines = File.ReadLines(path, System.Text.Encoding.UTF8);

            foreach (var line in lines)
            {
                Console.WriteLine(line);
            }

            // Regex
            var rx = new Regex(@"^[a-z]*$", RegexOptions.Compiled);
            var test = "input";

            if (rx.IsMatch(test))
            {
                Console.WriteLine("Is Match!");
            }
            else
            {
                Console.WriteLine("Is NOT Match!");
            }

        }
    }
}
