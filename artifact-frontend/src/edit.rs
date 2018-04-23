/* artifact: the requirements tracking tool made for developers
 * Copyright (C) 2018  Garrett Berg <@vitiral, vitiral@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the Lesser GNU General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the Lesser GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 * */
use dev_prelude::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Props {
    pub shared: Option<Arc<ProjectSer>>,
    pub name: Option<Name>,
}

/// Editable Artifact
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactEdit {
    pub shared: Arc<ProjectSer>,
    pub original: Option<ArtifactSer>,
    pub original_id: Option<HashIm>,
    pub name: String,
    // pub file: String,
    pub partof: Vec<String>,
    pub done: String,
    pub text: String,
}

pub enum StrField {
    Name,
    // File,
    Done,
    Text,
}

pub enum Msg {
    SetStr(StrField, String),
}


impl Component<super::Context> for ArtifactEdit {
    type Msg = Msg;
    type Properties = Props;

    fn create(p: Self::Properties, _: &mut Env<super::Context, Self>) -> Self {
        let shared = p.shared.expect("artifact: shared does not exist");
        match artifact {
            Some(a) => {
                ArtifactEdit {
                    shared,
                    original: Some(a.clone()),
                    original_id: Some(a.id.clone()),
                    name: a.name.to_string(),
                    // file: a.file.to_stfu8(),
                    partof: a.partof.iter().map(|n| n.to_string()).collect(),
                    done: a.impl_.as_done().map(String::from).unwrap_or_else(String::new),
                    text: a.text.clone(),
                }
            }
            None => {
                ArtifactEdit {
                    shared,
                    original: None,
                    original_id: None,
                    name: "".into(),
                    // file: a.file.to_stfu8(),
                    partof: Vec::new(),
                    done: "".into(),
                    text: "".into(),
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<super::Context, Self>) -> ShouldRender {
        match msg {
            Msg::SetStr(field, value) => {
                match field {
                    StrField::Name => self.name = value,
                    // StrField::File => self.file = value,
                    StrField::Done => self.done = value,
                    StrField::Text => self.text = value,
                }
            }
        }
        true
    }
}

fn view_parts(artifact: &ArtifactSer) -> Html<super::Context, ArtifactEdit> {
    let view_part = |name: &Name| html![
        <li>{name}</li>
    ];
    html![
        <ul>{ for artifact.parts.iter().map(view_part) }</ul>
    ]
}

impl Renderable<super::Context, ArtifactEdit> for ArtifactEdit {
    fn view(&self) -> Html<super::Context, Self> {
        let artifact = expect!(self.original.as_ref(), "TODO");
        html![
            <div>
              <h1>{ format!("Artifact: {}", artifact.name) }</h1>
              <p><b>{"Parts:"}</b></p>
              { view_parts(&artifact) }
            </div>
            // <h1>{ format!("Editable: {}", self.name) }</h1>
            // <h1>
            //   <input value=&self.name,
            //      oninput=|e: InputData| Msg::SetStr(StrField::Name, e.value),
            //   />
            // </h1>
        ]
    }
}