pub struct Fortune {
    pub id: i32,
    pub message: String,
}

markup::define! {
    Define<'a>(fortunes: &'a [Fortune]) {
        @let fortunes = *fortunes;
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }

    }
}

pub fn dynamic(fortunes: &[Fortune]) -> impl std::fmt::Display + '_ {
    markup::dynamic! {
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }

    }
}

pub fn to_string(fortunes: &[Fortune]) -> String {
    markup::to_string! {
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }

    }
}

pub fn to_writer(fortunes: &[Fortune], writer: &mut impl std::fmt::Write) -> std::fmt::Result {
    markup::to_writer! {
        writer =>
        {markup::doctype()}
        html {
            head {
                title { "Fortunes" }
            }
            body {
                table {
                    tr { th { "id" } th { "message" } }
                    @for item in fortunes {
                        tr {
                            td { @item.id }
                            td { @item.message }
                        }
                    }
                }
            }
        }

    }
}

#[test]
fn t() {
    let fortunes = [
        Fortune {
            id: 1,
            message: "fortune: No such file or directory".into(),
        },
        Fortune {
            id: 2,
            message: "A computer scientist is someone who fixes things that aren\'t broken.".into(),
        },
        Fortune {
            id: 3,
            message: "After enough decimal places, nobody gives a damn.".into(),
        },
        Fortune {
            id: 4,
            message: "A bad random number generator: 1, 1, 1, 1, 1, 4.33e+67, 1, 1, 1".into(),
        },
        Fortune {
            id: 5,
            message: "A computer program does what you tell it to do, not what you want it to do."
                .into(),
        },
        Fortune {
            id: 6,
            message: "Emacs is a nice operating system, but I prefer UNIX. — Tom Christaensen"
                .into(),
        },
        Fortune {
            id: 7,
            message: "Any program that runs right is obsolete.".into(),
        },
        Fortune {
            id: 8,
            message: "A list is only as strong as its weakest link. — Donald Knuth".into(),
        },
        Fortune {
            id: 9,
            message: "Feature: A bug with seniority.".into(),
        },
        Fortune {
            id: 10,
            message: "Computers make very fast, very accurate mistakes.".into(),
        },
        Fortune {
            id: 11,
            message:
                "<script>alert(\"This should not be displayed in a browser alert box.\");</script>"
                    .into(),
        },
        Fortune {
            id: 12,
            message: "フレームワークのベンチマーク".into(),
        },
    ];

    let define = Define {
        fortunes: &fortunes,
    }
    .to_string();
    let dynamic = dynamic(&fortunes).to_string();
    let to_string = to_string(&fortunes);
    let to_writer = {
        let mut string = String::new();
        to_writer(&fortunes, &mut string).unwrap();
        string
    };

    assert_eq!(define.len(), 1153);
    assert_eq!(define, dynamic);
    assert_eq!(define, to_string);
    assert_eq!(define, to_writer);
}

#[test]
fn to_writer_type_inference_bug() {
    let mut string = String::new();
    markup::to_writer!(&mut string => div {}).unwrap();
    assert_eq!(string, "<div></div>");
}