data = open("input/day2.txt").read().split('\n')


def is_valid_report(report):
    safe=True
    increasing = report[0] < report[1]
    for curr in range(1, len(report)):
        prev = curr-1
        diff = abs(report[prev]-report[curr])
        if diff < 1 or diff > 3:
            safe = False
            break
        if increasing != (report[prev] < report[curr]):
            safe = False
            break
        
    if safe:
        return True
    
    for i in range(len(report)):
        safe=True
        sub_report = report[:i] + report[i+1:]
        increasing = sub_report[0] < sub_report[1]
        for curr in range(1, len(sub_report)):
            prev = curr-1
            diff = abs(sub_report[prev]-sub_report[curr])
            if diff < 1 or diff > 3:
                safe = False
                break
            if increasing != (sub_report[prev] < sub_report[curr]):
                safe = False
                break
        if safe:
            return True
        
    return False

n_safe_reports = 0
for report in data:
    report = [int(n) for n in report.split()]
    if is_valid_report(report):
        print(' '.join(map(str, report)))
        n_safe_reports += 1
        
print(n_safe_reports)
    
        