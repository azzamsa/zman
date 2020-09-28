from datetime import datetime, tzinfo, timedelta


class UTC(tzinfo):
    """UTC"""

    def utcoffset(self, dt):
        return timedelta(0)

    def tzname(self, dt):
        return "UTC"

    def dst(self, dt):
        return timedelta(0)


def compute_progress(start, end, current):
    assert isinstance(start, datetime)
    assert isinstance(end, datetime)
    assert isinstance(current, datetime)

    whole_diff = end - start
    whole_diff_in_seconds = whole_diff.days * 86400 + whole_diff.seconds
    if whole_diff_in_seconds == 0:
        raise ValueError("Start and end datetimes are equal.")
    current_diff = current - start
    current_diff_in_seconds = current_diff.days * 86400 + current_diff.seconds
    return float(current_diff_in_seconds) / float(whole_diff_in_seconds)


def compute_current_year_progress(current=None):
    if not current:
        current = datetime.now(tz=UTC())
        start = datetime(current.year, 1, 1, tzinfo=UTC())
        end = datetime(current.year + 1, 1, 1, tzinfo=UTC())
    return compute_progress(start, end, current)


progress_width = 20
progress_block = "▓"
remain_block = "░"


def create_progress_string(progress, width=20):
    progress_int = int(round(progress * width))
    rest_int = int(width - progress_int)
    return "{}{}".format(progress_block * progress_int, remain_block * rest_int)


def year_progress():
    progress_ratio = compute_current_year_progress()
    print(progress_ratio)
    ratio_int = int(progress_ratio * 100)
    print(ratio_int)
    progress_bar = create_progress_string(progress_ratio, width=progress_width)

    # data = {"progress_bar": progress_bar, "ratio": ratio_int}
    # status = print(fmt, data)
    print(f"{progress_bar} {ratio_int}%")

    # return {"full_text": status}


year_progress()
