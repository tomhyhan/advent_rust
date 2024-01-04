#include "call_lib.h"

typedef struct {
    int64_t arr[50];
    size_t size;
} report_t;

report_t* parse(char* input) {
    int offset = 0, read;
    report_t* reports = create_list(report_t);

    char* token;
    char delim[] = "\n" ;

    token = strtok(input, delim);
    while (token) {
        int64_t num;
        char* end;
        int offset = 0, read;
        report_t report = {0};
        while (sscanf(token+offset, "%lld%n", &num, &read) == 1) {
            report.arr[report.size++] = num;
            offset += read;
        }
        push_list(reports, report);
        token = strtok(NULL, delim);
    }
    return reports;
}


void print_report(report_t report) {
    size_t i;
    for(i=0; i< report.size; i++) {
        printf("%lld ", report.arr[i]);
    }
    puts("");
}

bool is_all_zeros(report_t report) {
    size_t i;

    for (i=0; i<report.size; i++) {
        if (report.arr[i] != 0) return FALSE;
    }
    return TRUE;    
}


report_t add_to_last(report_t report) {
    size_t i;
    report_t next_report = {0};

    if (is_all_zeros(report)) {
        return report;
    }
    
    for (i=1; i<report.size; i++) {
        int64_t diff = report.arr[i] - report.arr[i-1];
        next_report.arr[next_report.size++] = diff; 
    }

    next_report = add_to_last(next_report);
    report.arr[report.size] = report.arr[report.size-1] + next_report.arr[next_report.size-1];
    report.size++;
    return report;
}

int64_t optimized(report_t report, bool part1) {
    report_t next_report = {0};
    size_t i;
    size_t report_idx = part1? report.size-1: 0; 
    size_t order = part1? 1: -1; 
    
    if (is_all_zeros(report)) {
        return 0;
    }
    
    for (i=1; i<report.size; i++) {
        int64_t diff = report.arr[i] - report.arr[i-1];
        next_report.arr[next_report.size++] = diff; 
    }
    
    return report.arr[report_idx] +  order * optimized(next_report, part1);
}

void part1(char* input) {
    report_t* reports = parse(input);
    size_t i;
    int64_t result = 0;
    int64_t result_op = 0;
    
    for (i=0; i < list_len(reports); i++) {
        report_t report = reports[i];
        int64_t last = optimized(report, TRUE);
        report = add_to_last(report);
        result += report.arr[report.size-1];
        result_op += last;
    }

    printf("%lld\n", result);
    printf("%lld\n", result_op);
    destroy_list(reports);
}

void move_report(report_t* report) {
    size_t i;
    report->size++;
    for(i=report->size-1;i>0;i--) {
        report->arr[i] = report->arr[i-1]; 
    }
}

report_t add_to_first(report_t report) {
    size_t i;
    report_t next_report = {0};
    if (is_all_zeros(report)) {
        return report;
    }
    
    for (i=1; i<report.size; i++) {
        int64_t diff = report.arr[i] - report.arr[i-1];
        next_report.arr[next_report.size++] = diff; 
    }

    next_report = add_to_first(next_report);
    move_report(&report);
    report.arr[0] = report.arr[1] - next_report.arr[0];
    return report;
}


void part2(char* input) {
    report_t* reports = parse(input);
    size_t i;
    int64_t result = 0;
    int64_t result_op = 0;

    for (i=0; i < list_len(reports); i++) {
        report_t report = reports[i];
        int64_t first = optimized(report, FALSE);
        report = add_to_first(report);
        result_op += first;
        result += report.arr[0];
    }

    printf("%lld\n", result);
    printf("%lld\n", result_op);
    destroy_list(reports);
}

SOLUTION("./inputs/q9.txt")
