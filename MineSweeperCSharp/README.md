# Minesweeper C# & .NET code

### Design Considerations & Assumptions

* The app is modularized and divided into 3 projects:
  
  * Minesweeper.Console — The console game application.
  
  * Minesweeper.Engine — The game library containing the core game logic. It can be reused and implemented for other application types, especially with a UI like web, desktop, and mobile, in addition to console.
  
  * Minesweeper.Tests — Contains the unit tests and E2E tests.

* Tried to keep the solution simple, straightforward, and easy to understand as much as possible with the principle of KISS and still adhering to the core concepts of clean code, object-oriented, and SOLID principles — when applicable.

* Created unit tests and E2E tests, though not exhaustive, they focus on the essential aspects of the logic and app flow.

* Made some minor tweak in the messages to make it easy for users to undertand the expected valid input values, such as min and max grid size.

* Added minor touches in grid rendering to include visual cues like text color and ensure the grid spacing remains uniform for large grid sizes when the cell number is two digits.

* In terms of performance and algorithm complexity, a balance of performance and solution simplicity was considered.
  
  * The app processing is simple enough that it will not incur noticeable performance impact, even for the largest possible grid and random mine placement, on any modern machine running it.
  
  * The randomization of mine placement in `Grid.PlaceMines()` and checking game completion by scanning each cell in `Game.IsGameWon()` are areas that could be considered for optimization, after certain benchmarking is performed, though they may not warrant it at this point.

* For the `Game.IsGameWon()` cell scanning, a certain optimization is used by checking the `Completed` flag cache and returns or it will break the loop once an unrevealed cell is found.
  
  * In the future, it can be further optimized, for instance by keeping track of revealed cells plus the number mines to determine game completion.
  
  * Or use a `HashSet<Point>` to keep track of unrevealed non-mine cells, removing cells as they are revealed, and considering the game completed once the `Set` is empty. Using a `Set` is efficient as it typically runs in average constant time, O(1).

* Coding style was fairly relaxed and did not enforce a strict coding standard since the app is simple.

### Project details

* Framework and Language: .NET 8 and C#
* Environment required: Cross-platform — Windows, Mac, Linux
* IDE or Editor: Visual Studio 2022 or VSCode (or JetBrains Rider)
* Testing library: NUnit

### Running the C# app from Command Line

To run the .NET console game application using the command line, you need to have the .NET SDK installed on your machine, see the [.NET website](https://dotnet.microsoft.com/en-us/).

#### 1. Open Command Line

Open a command line interface on your system:

- **Windows**: You can use Terminal, Command Prompt, or PowerShell.
- **macOS/Linux**: Open Terminal.

#### 2. Navigate to the Project Directory

```bash
cd Minesweeper\MinesweeperCSharp\Minesweeper.Console
```

#### 3. Build the Project

```bash
dotnet build
```

#### 4. Run the Application

```bash
dotnet run
```

This command will also compile (if necessary) and run the application directly.

If you prefer to run the built executable directly, navigate to the build output folder, and run the executable:

```bash
cd bin\Debug\net8.0
dotnet Minesweeper.Console.dll
```

### Running the C# tests from Command Line

#### 1. Navigate to the Tests Project Directory

Change to the directory that contains the tests project.

```bash
cd Minesweeper\MinesweeperCSharp\Minesweeper.Tests
```

#### 2. Build the Tests Project

Although this step is optional (since `dotnet test` will build the test project if it's not already built), you can explicitly compile the tests project to check for any compilation issues beforehand:

```bash
dotnet build
```

#### 3. Run the Tests

```bash
dotnet test
```

If you want to see more detailed output, you can use the verbosity option:

```bash
dotnet test --verbosity detailed
```
