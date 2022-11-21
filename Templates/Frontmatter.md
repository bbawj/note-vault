<%*
  let title = tp.file.title
  if (title.startsWith("Untitled")) {
    title = await tp.system.prompt("Title");
    await tp.file.rename(`${title}`);
  } 
-%>
---
title: "<%*tR+=`${title}`%>"
date: <% tp.file.creation_date("YYYY-MM-DD") %>
---
<% tp.file.cursor() %>