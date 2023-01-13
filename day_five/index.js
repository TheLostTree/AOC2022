//camel case to snake case json recursively
function camelToSnakeCase(obj) {
    if (typeof obj === 'object') {
        if (Array.isArray(obj)) {
        return obj.map(camelToSnakeCase);
        }
        return Object.keys(obj).reduce((acc, key) => {
        acc[key.replace(/([A-Z])/g, '_$1').toLowerCase()] = camelToSnakeCase(obj[key]);
        return acc;
        }, {});
    }
    return obj;
}

console.log(camelToSnakeCase({ helloWorld: 'hello', helloWorld2: { helloWorld3: 'hello' } }))
