using System.Text.RegularExpressions;

class Program
{
    private static void Main(string[] args)
    {
        try
        {
            StreamReader sr = new StreamReader("input.txt");

            int[,] grid1 = new int[1000, 1000];
            int[,] grid2 = new int[1000, 1000];

            for (string? line = sr.ReadLine(); line != null; line = sr.ReadLine())
            {
                Regex pattern = new Regex("^(turn on|turn off|toggle) ([0-9]{1,3}),([0-9]{1,3}) through ([0-9]{1,3}),([0-9]{1,3})$");
                Match match = pattern.Match(line);

                if (match.Success)
                {
                    // Bounding box
                    int x1 = int.Parse(match.Groups[2].Value), y1 = int.Parse(match.Groups[3].Value);
                    int x2 = int.Parse(match.Groups[4].Value), y2 = int.Parse(match.Groups[5].Value);

                    for (int i = x1; i <= x2; i++)
                    {
                        for (int j = y1; j <= y2; j++)
                        {
                            switch (match.Groups[1].Value)
                            {
                                case "turn on":
                                    grid1[i, j] = 1;
                                    grid2[i, j] += 1;
                                    break;
                                case "turn off":
                                    grid1[i, j] = 0;
                                    grid2[i, j] = (grid2[i, j] <= 0) ? 0 : grid2[i, j] - 1;
                                    break;
                                case "toggle":
                                    grid1[i, j] = (grid1[i, j] == 0) ? 1 : 0;
                                    grid2[i, j] += 2;
                                    break;
                                default:
                                    Console.WriteLine("Something happened: " + match.Groups[1].Value);
                                    Environment.Exit(1);
                                    break;
                            }
                        }
                    }
                }
                else
                {
                    Console.WriteLine("Bad instruction: " + line);
                    Environment.Exit(1);
                }
            }

            sr.Close();

            int count1 = 0;
            int count2 = 0;

            foreach (int i in grid1) count1 += i;
            foreach (int i in grid2) count2 += i;

            Console.WriteLine("Part 1 is: " + count1);
            Console.WriteLine("Part 2 is: " + count2);
        }
        catch (Exception e)
        {
            Console.WriteLine("Couldn't open input: " + e.Message);
        }
    }
}