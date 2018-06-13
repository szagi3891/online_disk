export interface NodeItemType {
    readonly is_dir: boolean,
    readonly hash: string,
}

export interface CurrentHead {
    head: string,
    counter: number,
};
