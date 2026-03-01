# Course Creation Task

Build a complete course from a topic brief.

## Steps

1. Parse the request for: topic, target audience, level, desired length
2. Delegate initial research to `researcher` agent
3. Design course outline:
   - Define 5-8 modules with clear progression
   - List 3-5 lessons per module with learning objectives
   - Identify practical exercises and projects
4. For each module:
   a. Write lesson content (800-1500 words per lesson)
   b. Include 2-3 examples per lesson
   c. Add practice exercise at end of each lesson
   d. Delegate quiz creation to `quiz_maker` agent
5. Create final assessment (15-20 questions)
6. Write course introduction and prerequisites
7. Compile course into organized file structure:
   - README.md (course overview)
   - module-01/ through module-XX/
   - Each with lesson files and quiz files
8. Send course package via email
9. Save to memory (tag "course:[topic]")
