use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
}

#[function_component(MarkdownViewer)]
pub fn markdown_viewer(props: &Props) -> Html {
    let parsed_content = parse_markdown(&props.content);

    html! {
        <div class="markdown-viewer">
            {
                parsed_content.into_iter().map(|segment| {
                    match segment {
                        MarkdownSegment::Bold(text) => html! { <strong>{ text }</strong> },
                        MarkdownSegment::Italic(text) => html! { <em>{ text }</em> },
                        MarkdownSegment::Plain(text) => html! { <span>{ text }</span> },
                        MarkdownSegment::Newline => html! { <br /> },
                        MarkdownSegment::BulletPoint(text) => html! { <li>{ text }</li> },
                    }
                }).collect::<Html>()
            }
        </div>
    }
}


enum MarkdownSegment {
    Bold(String),
    Italic(String),
    Plain(String),
    Newline,
    BulletPoint(String),
}

fn parse_markdown(input: &str) -> Vec<MarkdownSegment> {
    let mut segments = Vec::new();
    let mut last_end = 0;

    for (start, end, marker) in find_markdown_segments(input) {
        // Add plain text before the styled segment
        if start > last_end {
            segments.extend(process_plain_text(&input[last_end..start]));
        }

        // Add the styled segment
        match marker {
            "**" => segments.push(MarkdownSegment::Bold(input[start + 2..end - 2].trim().to_string())),
            "*" => segments.push(MarkdownSegment::Italic(input[start + 1..end - 1].trim().to_string())),
            _ => {}
        }

        last_end = end;
    }

    // Add remaining plain text
    if last_end < input.len() {
        segments.extend(process_plain_text(&input[last_end..]));
    }

    segments
}

fn process_plain_text(text: &str) -> Vec<MarkdownSegment> {
    text.split('\n').flat_map(|line| {
        if line.trim_start().starts_with('*') {
            // Handle bullet points
            let bullet_text = line.trim_start().trim_start_matches('*').trim();
            vec![MarkdownSegment::BulletPoint(bullet_text.to_string())]
        } else {
            // Add plain text with line breaks
            let mut result = Vec::new();
            if !line.trim().is_empty() {
                result.push(MarkdownSegment::Plain(line.trim().to_string()));
            }
            result.push(MarkdownSegment::Newline);
            result
        }
    }).collect()
}

fn find_markdown_segments(input: &str) -> Vec<(usize, usize, &'static str)> {
    let mut segments = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((start, c)) = chars.next() {
        let marker = match c {
            '*' if chars.peek().map(|(_, c)| *c) == Some('*') => {
                chars.next(); // Consume the second '*'
                "**"
            }
            '*' => "*",
            _ => continue,
        };

        if let Some((end, _)) = chars.find(|(_, c)| *c == marker.chars().next().unwrap()) {
            if marker == "**" {
                if chars.peek().map(|(_, c)| *c) == Some('*') {
                    chars.next(); // Consume the second '*'
                    segments.push((start, end + 2, marker));
                }
            } else {
                segments.push((start, end + 1, marker));
            }
        }
    }

    segments
}

