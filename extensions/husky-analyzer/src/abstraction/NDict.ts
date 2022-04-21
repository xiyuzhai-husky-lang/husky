export default class NDict<T> {
    data: { [key: number]: T } = {};

    clear() {
        this.data = {};
    }

    insert_new(key: number, value: T) {
        if (key in this.data) {
            throw new Error(
                `can't insert new ${key}, because it's already in the data with value = ${JSON.stringify(
                    this.data[key]
                )}`
            );
        }
        this.data[key] = value;
    }

    get(key: number) {
        console.assert(key in this.data);
        return this.data[key];
    }

    print_state(indent: number) {
        console.log(" ".repeat(indent), "{");
        for (const [key, value] of Object.entries(this.data)) {
            console.log(" ".repeat(indent + 4), "key ", key, ": ", value);
        }
        console.log(" ".repeat(indent), "}");
    }
}
