export default {
    "*.{js?(x),(le|c|sc|sa)ss,md,json,vue}": [
        "node node_modules/prettier/bin/prettier.cjs --write",
    ],
    "*.ts?(x)": [
        "node node_modules/prettier/bin/prettier.cjs --parser=typescript --write",
    ],
};
