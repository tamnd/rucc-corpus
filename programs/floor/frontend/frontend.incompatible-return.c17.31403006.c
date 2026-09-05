static int give(void) {
    struct empty { int value; } made = { 1 };
    return made;
}
int main(void) {
    return give();
}
