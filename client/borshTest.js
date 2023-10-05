const borsh = require('borsh');

class Test {
    constructor(value = 0) {
        this.value = value;
    }
}

const TestSchema = new Map([
    [Test, {kind: 'struct', fields: [['value', 'u32']]}]
]);

const test = new Test(123);

console.log('Test instance:', test);
console.log('Test schema:', TestSchema);

try {
    const serializedData = borsh.serialize(TestSchema, test);
    console.log('Serialized:', serializedData);
    const deserializedData = borsh.deserialize(TestSchema, Test, serializedData);
    console.log('Deserialized:', deserializedData);
} catch (error) {
    console.error('Borsh Test Error: ', error);
}
