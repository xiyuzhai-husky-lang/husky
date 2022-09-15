structure Subsection

structure Section where
  title : String
  subsections: List Subsection

structure Chapter where
  title : String
  sections : List Section

structure Book where
  title : String
  chapters: List Chapter