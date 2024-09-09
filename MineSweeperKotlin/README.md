# Minesweeper Kotlin

The Kotlin app was created after completing the C# code; therefore, for time considerations, it was organized more simply in one module or project.

### Project details

* SDK and Language: OpenJDK 22 and Kotlin
* Environment required: Cross-platform — Windows, Mac, Linux
* IDE or Editor: IntelliJ IDEA
* Testing library: JUnit 5

### Running `Main.kt` from IntelliJ IDEA

1. Start IntelliJ IDEA and open the `Minesweeper\MinesweeperKotlin` project.

2. Find `Main.kt` in the Project Explorer on the left side of the IDE.

3. Run the Main Function:
   
   - Right-click on the `Main.kt` file in the Project Explorer.
   - Choose **Run 'MainKt'** from the context menu.
   - Alternatively, open the `Main.kt` file in the editor, and you should see a green play button next to the `main` function line. Click on the green play button and select **Run 'MainKt'** from the pop-up.
   
   IntelliJ will compile the code and run the `main()` function, displaying the game output in the Run window at the bottom of the IDE.

### Running the Tests from IntelliJ IDEA

1. If not already open, start IntelliJ IDEA and open the project.

2. In the Project Explorer, navigate to the test files. These are located in the `src/test/kotlin` directory.

3. To run all the tests in the project, right-click on the test directory and choose **Run 'All Tests'**.
   
   Results will be displayed in the Run window, where each test's status (passed, failed, ignored) is clearly marked.
