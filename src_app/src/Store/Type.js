//@flow

export type NodeItemType = {|
    +is_dir: bool,
    +hash: string,
|};

export type CurrentHead = {|
    head: string,
    counter: number,
|};
