use crate::{Dialog, Error, OpenMultipleFile, OpenSingleFile, Result};
use osascript::JavaScript;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl Dialog for OpenSingleFile<'_> {
    type Output = Option<String>;

    fn show(self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: false,
            dir: self.dir,
            filter: self.filter,
        })
    }
}

impl Dialog for OpenMultipleFile<'_> {
    type Output = Vec<String>;

    fn show(self) -> Result<Self::Output> {
        choose_file(ChooseFileParams {
            multiple: true,
            dir: self.dir,
            filter: self.filter,
        })
    }
}

#[derive(Serialize)]
struct ChooseFileParams<'a> {
    multiple: bool,
    dir: Option<&'a str>,
    filter: Option<&'a [&'a str]>,
}

fn choose_file<T: DeserializeOwned>(params: ChooseFileParams) -> Result<T> {
    let script = JavaScript::new(
        // language=js
        r"
        const app = Application.currentApplication();
        app.includeStandardAdditions = true;

        const options = {
            multipleSelectionsAllowed: $params.multiple,
        };

        if ($params.dir)
            options.defaultLocation = Path($params.dir.replace(/^\~/, app.pathTo('home folder')));

        if ($params.filter)
            options.ofType = $params.filter;

        try {
            let path = app.chooseFile(options);
            
            if ($params.multiple)
                return path.map(x => x.toString());
            else 
                return path.toString();
        } catch (e) {
            return null;
        }
        ",
    );

    script.execute_with_params(params).map_err(Error::from)
}
