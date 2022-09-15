structure Subsection

structure Section where
  subsections: List Subsection

structure Chapter where
  sections : List Section

structure Book where
  chapters: List Chapter